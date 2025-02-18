use std::{
    collections::HashMap,
    env,
    error::Error,
    io::Write,
    path::PathBuf,
    process::Command,
    sync::{Arc, Mutex},
    time::Instant,
    vec,
};

use db::initialize_database;
use message::save;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    ModelTrait, QueryFilter, Set,
};

pub mod db;
pub mod gpt;
pub mod models;
pub mod srv;

use models::{
    ienum::MultiNodeType,
    prompter::{Edge, Node},
    *,
};
use serde::{Deserialize, Serialize};
use tauri::{
    generate_context, ipc::Channel, window, AppHandle, Builder, Emitter, Manager,
    WebviewWindowBuilder,
};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use tempfile::NamedTempFile;

static mut DB: Option<DatabaseConnection> = None;
static mut GPT_CONFIG: Option<gpt::GptConfig> = None;

async fn get_gpt_models() -> Result<gpt::GptConfig, String> {
    unsafe { Ok(GPT_CONFIG.clone().unwrap()) }
}

#[tauri::command]
async fn get_models() -> Result<gpt::GptConfig, String> {
    Ok(get_gpt_models().await.unwrap())
}

#[tauri::command]
async fn greet() -> String {
    unsafe {
        let db = DB.clone().unwrap();
        message::ActiveModel {
            id: Set(0),
            session_uuid: Set(uuid::Uuid::new_v4().to_string()),
            order: Set(0),
            content: Set("Hello, world!".to_string()),
            itype: Set(ienum::MessageType::USER.to_string()),
        }
        .insert(&db)
        .await
        .unwrap();
    }
    "Hello, world!".to_string()
}

#[tauri::command]
async fn get_prompter(uuid: String) -> Result<prompter::Model, String> {
    get_prompter_item(uuid).await
}

async fn get_prompter_item(uuid: String) -> Result<prompter::Model, String> {
    unsafe {
        let db = DB.clone().unwrap();
        let a = prompter::Entity::find_by_id(uuid).one(&db).await.unwrap();
        Ok(a.unwrap())
    }
}

#[tauri::command]
async fn get_prompters() -> Result<Vec<prompter::Model>, String> {
    unsafe {
        let db = DB.clone().unwrap();
        let a = prompter::Entity::find().all(&db).await.unwrap();
        Ok(a)
    }
}
#[tauri::command]
async fn save_prompter(prompter: prompter::Model) -> Result<prompter::Model, String> {
    unsafe {
        let db = DB.clone().unwrap();
        let mut am = prompter::ActiveModel {
            uuid: Set(prompter.uuid.clone()),
            title: Set(prompter.title),
            content: Set(prompter.content),
            itype: Set(prompter.itype),
            model_name: Set(prompter.model_name),
            ptype: Set(prompter.ptype),
            nodes: Set(prompter.nodes),
            edges: Set(prompter.edges),
        };
        let a: prompter::Model;

        if prompter.uuid.is_empty() {
            am.uuid = Set(uuid::Uuid::new_v4().to_string());
            a = am.insert(&db).await.unwrap();
        } else {
            a = am.update(&db).await.unwrap();
        }

        Ok(a)
    }
}

#[tauri::command]
async fn get_sessions() -> Result<Vec<session::Model>, String> {
    unsafe {
        let db = DB.clone().unwrap();
        let a = session::Entity::find().all(&db).await.unwrap();
        Ok(a)
    }
}

#[tauri::command]
async fn save_session(session: session::Model) -> Result<session::Model, String> {
    unsafe {
        let db = DB.clone().unwrap();
        let mut am = session::ActiveModel {
            uuid: Set(session.uuid.clone()),
            prompter_uuid: Set(session.prompter_uuid),
            session_type: Set(session.session_type),
            session_title: Set(session.session_title),
        };
        let a: session::Model;

        // if session.uuid.is_empty() {
        // am.uuid = Set(uuid::Uuid::new_v4().to_string());
        a = am.insert(&db).await.unwrap();
        // } else {
        //     am.save(&db).await.unwrap();
        // }

        Ok(a)
    }
}

#[tauri::command]
async fn get_messages(session_uuid: String) -> Result<Vec<message::Model>, String> {
    println!("get session_uuid: {}", session_uuid);
    unsafe {
        let db = DB.clone().unwrap();
        let a = message::Entity::find()
            .filter(message::Column::SessionUuid.eq(session_uuid))
            .all(&db)
            .await
            .unwrap();

        Ok(a)
    }
}

#[tauri::command]
async fn save_message(
    prompter: prompter::Model,
    message_item: message::Model,
) -> Result<String, String> {
    let messages = get_messages(message_item.session_uuid.clone())
        .await
        .unwrap();

    let mut gpt_messages = vec![];

    gpt_messages.push(gpt::GptMessage {
        role: "system".to_string(),
        content: "every things return markdown syntax".to_string(),
    });

    if messages.len() == 0 {
        // save system
        unsafe {
            let db = DB.clone().unwrap();
            save(
                &db,
                message::Model {
                    id: 0,
                    session_uuid: message_item.session_uuid.clone(),
                    order: 0,
                    content: prompter.content.clone(),
                    itype: ienum::MessageType::SYSTEM.to_string(),
                },
            )
            .await
            .unwrap();
        }

        gpt_messages.push(gpt::GptMessage {
            role: ienum::MessageType::SYSTEM.to_string(),
            content: prompter.content.clone(),
        });
        gpt_messages.push(gpt::GptMessage {
            role: ienum::MessageType::USER.to_string(),
            content: message_item.content.clone(),
        });
    }

    messages.iter().for_each(|m| {
        gpt_messages.push(gpt::GptMessage {
            role: m.itype.clone(),
            content: m.content.clone(),
        });
    });
    unsafe {
        let db = DB.clone().unwrap();
        message::save(&db, message_item.clone()).await.unwrap();
        db.close();
    }

    gpt_messages.push(gpt::GptMessage {
        role: ienum::MessageType::USER.to_string(),
        content: message_item.content.clone(),
    });

    // 需要区分模型类型
    let assistant_message = gpt::call_gpt(
        get_config()
            .models
            .iter()
            .find(|m| m.model == prompter.model_name)
            .unwrap()
            .clone(),
        &gpt_messages,
    )
    .await
    .unwrap();

    unsafe {
        let db = DB.clone().unwrap();
        message::save(
            &db,
            message::Model {
                id: 0,
                session_uuid: message_item.session_uuid.clone(),
                order: messages.len() as i32 + 1,
                content: assistant_message,
                itype: ienum::MessageType::ASSISTANT.to_string(),
            },
        )
        .await
        .unwrap();
        db.close();
    }

    Ok("".to_string())

    // 拿prompter
    // 将prompter作为SYSTEM传入
    // 将message作为USER传入
    // 调用gpt
    // 将gpt返回的message作为ASSISTANT传入
}
async fn save_dbmsg(message: message::Model) {
    unsafe {
        let db = DB.clone().unwrap();
        message::save(&db, message.clone()).await.unwrap();
        db.close();
    }
}

async fn handle_normal_prompter(
    prompter: prompter::Model,
    message_item: message::Model,
) -> Result<String, String> {
    let messages = get_messages(message_item.session_uuid.clone())
        .await
        .unwrap();

    let mut gpt_messages = vec![];

    gpt_messages.push(gpt::GptMessage {
        role: "system".to_string(),
        content: "every things return markdown syntax".to_string(),
    });

    if messages.len() == 0 {
        // save system
        unsafe {
            let db = DB.clone().unwrap();
            save(
                &db,
                message::Model {
                    id: 0,
                    session_uuid: message_item.session_uuid.clone(),
                    order: 0,
                    content: prompter.content.clone(),
                    itype: ienum::MessageType::SYSTEM.to_string(),
                },
            )
            .await
            .unwrap();
        }

        gpt_messages.push(gpt::GptMessage {
            role: ienum::MessageType::SYSTEM.to_string(),
            content: prompter.content.clone(),
        });
        gpt_messages.push(gpt::GptMessage {
            role: ienum::MessageType::USER.to_string(),
            content: message_item.content.clone(),
        });
    }

    messages.iter().for_each(|m| {
        gpt_messages.push(gpt::GptMessage {
            role: m.itype.clone(),
            content: m.content.clone(),
        });
    });
    unsafe {
        let db = DB.clone().unwrap();
        message::save(&db, message_item.clone()).await.unwrap();
        db.close();
    }

    gpt_messages.push(gpt::GptMessage {
        role: ienum::MessageType::USER.to_string(),
        content: message_item.content.clone(),
    });

    // 需要区分模型类型
    let assistant_message = gpt::call_gpt(
        get_config()
            .models
            .iter()
            .find(|m| m.model == prompter.model_name)
            .unwrap()
            .clone(),
        &gpt_messages,
    )
    .await
    .unwrap();

    unsafe {
        let db = DB.clone().unwrap();
        message::save(
            &db,
            message::Model {
                id: 0,
                session_uuid: message_item.session_uuid.clone(),
                order: messages.len() as i32 + 1,
                content: assistant_message,
                itype: ienum::MessageType::ASSISTANT.to_string(),
            },
        )
        .await
        .unwrap();
        db.close();
    }

    Ok("".to_string())

    // 拿prompter
    // 将prompter作为SYSTEM传入
    // 将message作为USER传入
    // 调用gpt
    // 将gpt返回的message作为ASSISTANT传入
}

#[tauri::command]
async fn handle_pyscript_prompter(
    prompter: prompter::Model,
    message_item: message::Model,
    bind_code: bool,
) -> Result<String, String> {
    let messages = get_messages(message_item.session_uuid.clone())
        .await
        .unwrap();

    let mut gpt_messages = vec![];

    gpt_messages.push(gpt::GptMessage {
        role: "system".to_string(),
        content: "just return python3 code,python3 just local run".to_string(),
    });

    // 加载 system promtper
    gpt_messages.push(gpt::GptMessage {
        role: ienum::MessageType::SYSTEM.to_string(),
        content: prompter.content.clone(),
    });
    gpt_messages.push(gpt::GptMessage {
        role: ienum::MessageType::USER.to_string(),
        content: message_item.content.clone(),
    });

    unsafe {
        let db = DB.clone().unwrap();
        message::save(&db, message_item.clone()).await.unwrap();
        db.close();
    }

    gpt_messages.push(gpt::GptMessage {
        role: ienum::MessageType::USER.to_string(),
        content: message_item.content.clone(),
    });

    let mut python_code = gpt::call_gpt(
        get_config()
            .models
            .iter()
            .find(|m| m.model == prompter.model_name)
            .unwrap()
            .clone(),
        &gpt_messages,
    )
    .await
    .unwrap();

    let temp_file = tempfile::Builder::new()
        .prefix("rust_python_") // 指定前缀
        .tempfile_in("/tmp") // 指定目录为 /tmp
        .expect("Failed to create a temporary file");

    // 获取临时文件的路径
    let temp_path = temp_file.path();

    // 将 Python 代码写入临时文件
    {
        let mut temp_file = temp_file.reopen().expect("Failed to reopen temporary file");
        temp_file
            .write_all(python_code.as_bytes())
            .expect("Failed to write to temporary file");
    }

    println!("python3 tmp file: {}", temp_path.display());

    let output = Command::new("python3")
        .arg(temp_path)
        .output()
        .expect("Failed to execute python3");

    // 执行python3代码
    if output.status.success() {
        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
        if bind_code {
            python_code = format!(
                "python code\n```python\n{}\n```\n>{}\n",
                python_code,
                String::from_utf8_lossy(&output.stdout)
            );
        } else {
            python_code = String::from_utf8_lossy(&output.stdout).to_string();
        }
    } else {
        // 失败就返回代码和错误的结果
        python_code = format!(
            "err py3 code\n\n```python\n{}\n```\n```\n{}\n```\n",
            python_code,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    unsafe {
        let db = DB.clone().unwrap();
        message::save(
            &db,
            message::Model {
                id: 0,
                session_uuid: message_item.session_uuid.clone(),
                order: messages.len() as i32 + 1,
                content: python_code,
                itype: ienum::MessageType::ASSISTANT.to_string(),
            },
        )
        .await
        .unwrap();
        db.close();
    }

    Ok("".to_string())

    // 拿prompter
    // 将prompter作为SYSTEM传入
    // 将message作为USER传入
    // 调用gpt
    // 将gpt返回的message作为ASSISTANT传入
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Step {
    uuid: String,
    content: String,
    ok: bool,
    msg: String,
    value: String,
    time_secs: i32,
}

#[tauri::command]
async fn handle_multi_prompter(
    app: AppHandle,
    prompter: prompter::Model,
    message: message::Model,
    channel: Channel<Step>,
) -> Result<String, String> {
    let edges = prompter.get_edges();
    let nodes = prompter.get_nodes();

    let nodes_map = nodes
        .iter()
        .map(|n| (n.id.clone(), n))
        .collect::<HashMap<String, &Node>>();

    let mut edgesMap: HashMap<String, Vec<Edge>> = HashMap::new();

    edges.iter().for_each(|e| {
        edgesMap
            .entry(e.source.clone())
            .or_insert(vec![])
            .push(e.clone());
    });

    println!("nodes: {:?}", nodes);
    println!("edges: {:?}", edges);
    save_dbmsg(message::Model {
        id: 0,
        session_uuid: message.session_uuid.clone(),
        order: 1,
        content: message.content.clone(),
        itype: ienum::MessageType::USER.to_string(),
    })
    .await;

    // ENTRY为起点，然后以ENTRY为起点走Edges的流程
    // 就是 ENTRY为始，ENTRY的TARGET，以及对应的 SPEC_VAR的TARGET，如果不符合条件都终止即可
    let mut headSteap = "ENTRY".to_string();
    let edges = edgesMap.get(&headSteap);
    if edges.is_none() {
        println!("edges is none: {}", headSteap);
        return Ok("".to_string());
    }

    let edges = edges.unwrap();
    headSteap = edges[0].target.clone();
    let mut res_msg = "".to_string();

    loop {
        let a = nodes_map.get(&headSteap);
        if a.is_none() {
            let _ = channel.send(Step {
                uuid: headSteap.clone(),
                content: "".to_string(),
                ok: false,
                msg: "failed to get next step node".to_string(),
                value: "".to_string(),
                time_secs: 0,
            });
            break;
        }

        let node = a.unwrap();
        if node.data.data.is_none() {
            println!("node data is none: {}", node.id);

            let _ = channel.send(Step {
                uuid: node.id.clone(),
                content: "".to_string(),
                ok: false,
                msg: "failed parse next step model data of node".to_string(),
                value: "".to_string(),
                time_secs: 0,
            });
            break;
        }
        let mut gpt_messages = vec![];
        let node_data = node.data.data.as_ref().unwrap();
        let prompter = get_prompter_item(node_data.prompter_uuid.clone())
            .await
            .unwrap();

        gpt_messages.push(gpt::GptMessage {
            role: "system".to_string(),
            content: prompter.content.clone(),
        });

        gpt_messages.push(gpt::GptMessage {
            role: ienum::MessageType::USER.to_string(),
            content: message.content.clone(),
        });

        let time_start = Instant::now();

        // 处理问题
        let assistant_message = gpt::call_gpt(
            get_config()
                .models
                .iter()
                .find(|m| m.model == prompter.model_name)
                .unwrap()
                .clone(),
            &gpt_messages,
        )
        .await
        .unwrap();

        let step = Step {
            uuid: node.id.clone(),
            content: message.content.clone(),
            ok: true,
            msg: "".to_string(),
            value: assistant_message.clone(),
            time_secs: time_start.elapsed().as_secs() as i32,
        };
        // 返回的信息
        res_msg = assistant_message.clone();
        println!("call gpt step: {:?}", step.time_secs);

        // message 存储到 db中
        gpt_messages.push(gpt::GptMessage {
            role: ienum::MessageType::ASSISTANT.to_string(),
            content: assistant_message.clone(),
        });

        let edges = edgesMap.get(&headSteap);
        if edges.is_none() {
            println!("edges is none: {}", headSteap);
            let _ = channel.send(step);
            break;
        }

        let _ = channel.send(step);

        let edges = edges.unwrap();
        let old_step = headSteap.clone();

        edges.iter().for_each(|e| {
            // println!("edge: {:?}", e);
            if e.data.is_none() {
                println!("edge data is none: {}", e.target);
                return;
            }

            let data = e.data.as_ref().unwrap();
            let tp = data.stype.clone();

            match tp.as_str() {
                "NORMAL" => {
                    headSteap = edges[0].target.clone();
                    println!("NORMAL");
                }
                "SPEC_VAR" => {
                    let spec_var = assistant_message.clone();

                    if data.cond_var == spec_var {
                        println!("SPEC_VAR: {} -> {}", spec_var, e.target);
                        headSteap = e.target.clone();
                    }
                }
                _ => {
                    app.dialog()
                        .message(format!("unimplmented node type: {}", tp))
                        .title("error")
                        .kind(MessageDialogKind::Error)
                        .blocking_show();
                }
            }
        });

        if headSteap == old_step {
            println!("headSteap == old_step: {}", headSteap);
            println!("save_msg_db: {}", res_msg);
            save_dbmsg(message::Model {
                id: 0,
                session_uuid: message.session_uuid.clone(),
                order: 2,
                content: res_msg.clone(),
                itype: ienum::MessageType::ASSISTANT.to_string(),
            })
            .await;
            return Ok("".to_string());
        }
    }

    println!("save_msg_db: {}", res_msg);
    save_dbmsg(message::Model {
        id: 0,
        session_uuid: message.session_uuid.clone(),
        order: 2,
        content: res_msg.clone(),
        itype: ienum::MessageType::ASSISTANT.to_string(),
    })
    .await;
    Ok("".to_string())
}

#[tauri::command]
async fn delete_prompter(uuid: String) -> Result<(), String> {
    unsafe {
        let db = DB.clone().unwrap();
        prompter::Entity::delete_by_id(uuid)
            .exec(&db)
            .await
            .unwrap();
        Ok(())
    }
}

fn get_config() -> gpt::GptConfig {
    unsafe { GPT_CONFIG.clone().unwrap() }
}

#[tauri::command]
async fn get_keyshuts() -> Result<Vec<keyshut::Model>, String> {
    unsafe {
        let db = DB.clone().unwrap();
        let a = keyshut::Entity::find().all(&db).await.unwrap();
        Ok(a)
    }
}

#[tauri::command]
async fn save_keyshut(keyshut: keyshut::Model) -> Result<keyshut::Model, String> {
    unsafe {
        let db = DB.clone().unwrap();
        let mut am = keyshut::ActiveModel {
            id: NotSet,
            keyshut: Set(keyshut.keyshut),
            prompter_uuid: Set(keyshut.prompter_uuid),
        };
        let a: keyshut::Model;
        match am.insert(&db).await {
            Ok(a) => Ok(a),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[tauri::command]
async fn delete_keyshut(id: i32) -> Result<(), String> {
    unsafe {
        let db = DB.clone().unwrap();
        let model = keyshut::Entity::find_by_id(id).one(&db).await.unwrap();

        if let Some(model) = model {
            model.delete(&db).await.unwrap();
            Ok(())
        } else {
            Err("Keyshut not found".to_string())
        }
    }
}

// #[tauri::command]
// async fn new_window(app: tauri::AppHandle) {
//     let window_id = uuid::Uuid::new_v4().to_string();
//     let puuid = "48ed2cf4-3d0a-464c-991c-2a7631c03b11".to_string();
//     let prompter = get_prompter_item(puuid.clone()).await.unwrap();

//     tauri::WebviewWindowBuilder::new(
//         &app,
//         window_id,
//         tauri::WebviewUrl::App(PathBuf::from(
//             format!("/popwin?puuid={}", puuid.clone()).to_string(),
//         )),
//     )
//     .title(prompter.title.clone())
//     .build()
//     .unwrap();
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    unsafe {
        DB = Some(
            initialize_database(
                format!("sqlite://{}/data.db?mode=rwc", env::var("HOME").unwrap()).as_str(),
            )
            .await
            .unwrap(),
        );

        match gpt::get_config() {
            Ok(config) => {
                GPT_CONFIG = Some(config.clone());
                println!("get config: {}", config.to_string());
            }
            Err(e) => panic!("get config {}", e),
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let gg = app.handle();
            // zbsd;
            unsafe {
                let srv = srv::MyApp::new(
                    "test".to_string(),
                    Arc::new(Mutex::new(gg.clone())),
                    DB.clone().unwrap(),
                );

                std::thread::spawn(move || {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async {
                        srv.start().await.unwrap();
                    });
                });
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            // new_window,
            save_message,
            get_messages,
            save_prompter,
            get_prompters,
            save_session,
            get_sessions,
            get_keyshuts,
            save_keyshut,
            delete_keyshut,
            get_prompter,
            get_models,
            delete_prompter,
            handle_multi_prompter,
            handle_pyscript_prompter,
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}
