use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sea_orm::{DatabaseConnection, EntityTrait};
use std::error::Error;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

use crate::models;

struct AppState {
    app: MyApp,
}
// 定义一个Trait，包含需要在HTTP请求中调用的方法
trait MyHandler {
    async fn handle_request(&self, uuid: String) -> Result<String, String>;
}

// 定义一个结构体
#[derive(Clone)]
pub struct MyApp {
    app_name: String,
    app: Arc<Mutex<tauri::AppHandle>>,
    db: DatabaseConnection,
}

// 为结构体实现Trait
impl MyHandler for MyApp {
    async fn handle_request(&self, puuid: String) -> Result<String, String> {
        let app = self.app.lock().unwrap();
        let db = self.db.clone();

        let prompter = models::prompter::Entity::find_by_id(puuid.clone()).one(&db);

        match prompter.await {
            Ok(prompter) => {
                let window_id = uuid::Uuid::new_v4().to_string();
                if prompter.is_none() {
                    app.dialog()
                        .message("Prompter not found")
                        .kind(MessageDialogKind::Error)
                        .title("Warning")
                        .blocking_show();

                    return Err("Prompter not found".to_string());
                }

                tauri::WebviewWindowBuilder::new(
                    &app.clone(),
                    window_id,
                    tauri::WebviewUrl::App(PathBuf::from(
                        format!("/popwin?puuid={}", puuid.clone()).to_string(),
                    )),
                )
                .title(prompter.unwrap().title)
                .build()
                .unwrap();

                Ok(format!("Hello {}!", "wqni["))
            }
            Err(e) => {
                app.dialog()
                    .message("File not found")
                    .kind(MessageDialogKind::Error)
                    .title("Warning")
                    .blocking_show();

                Err(e.to_string())
            }
        }
    }
}

// 实现结构体的方法
impl MyApp {
    // 构造函数
    pub fn new(
        app_name: String,
        app: Arc<Mutex<tauri::AppHandle>>,
        db: DatabaseConnection,
    ) -> Self {
        Self { app_name, app, db }
    }

    // 启动HTTP服务器
    pub async fn start(&self) -> std::io::Result<()> {
        let app_data = web::Data::new(Arc::new(AppState { app: self.clone() }));
        // 将结构体的引用包装在Arc中，以便在多线程中共享
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        println!("Server started on port: {}", port);
        let server = HttpServer::new(move || {
            App::new()
                .app_data(app_data.clone()) // 共享结构体数据
                .route("/{puuid}", web::get().to(Self::handle_http)) // 注册路由
        })
        .listen(listener)?
        .run()
        .await;
        match server {
            Ok(s) => Ok(()),
            Err(e) => {
                println!("{}", e);
                Ok(())
            }
        }

        // HttpServer::new(move || {
        //     App::new()
        //         .app_data(app_data.clone()) // 共享结构体数据
        //         .route("/{puuid}", web::get().to(Self::handle_http)) // 注册路由
        // })
        // .bind("127.0.0.1:0")?
        // .run()
        // .await
    }

    // 将Trait的方法包装为actix-web的处理函数
    async fn handle_http(
        data: web::Data<Arc<AppState>>,
        puuid: web::Path<String>,
    ) -> impl Responder {
        let app = data.app.clone();
        let response = app.handle_request(puuid.to_string()).await.unwrap();
        println!("{}", response);
        HttpResponse::Ok().body(response)
    }
}
