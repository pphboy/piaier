use sea_orm::ConnectionTrait;
use sea_orm::{Database, DatabaseConnection, DbErr, Schema, Statement};

use crate::models::*;

pub async fn initialize_database(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    // 创建或打开 SQLite 数据库文件
    let db = Database::connect(database_url).await?;

    // 检查表是否存在，如果不存在则创建表
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    if !has_table(&db, "message").await? {
        let stmt = builder.build(&schema.create_table_from_entity(message::Entity));
        db.execute(stmt).await?;
        println!("Table 'message' created successfully!");
    } else {
        println!("Table 'message' already exists.");
    }

    if !has_table(&db, "session").await? {
        let stmt = builder.build(&schema.create_table_from_entity(session::Entity));
        db.execute(stmt).await?;
        println!("Table 'session' created successfully!");
    } else {
        println!("Table 'session' already exists.");
    }

    if !has_table(&db, "prompter").await? {
        let stmt = builder.build(&schema.create_table_from_entity(prompter::Entity));
        db.execute(stmt).await?;
        println!("Table 'prompter' created successfully!");
    } else {
        println!("Table 'prompter' already exists.");
    }

    if !has_table(&db, "keyshut").await? {
        let stmt = builder.build(&schema.create_table_from_entity(keyshut::Entity));
        db.execute(stmt).await?;
        println!("Table 'keyshut' created successfully!");
    } else {
        println!("Table 'keyshut' already exists.");
    }

    if !has_table(&db, "mpstep").await? {
        let stmt = builder.build(&schema.create_table_from_entity(mpstep::Entity));
        db.execute(stmt).await?;
        println!("Table 'mpstep' created successfully!");
    } else {
        println!("Table 'mpstep' already exists.");
    }

    Ok(db)
}

async fn has_table(db: &DatabaseConnection, table_name: &str) -> Result<bool, DbErr> {
    let query = format!(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='{}';",
        table_name
    );
    let result = db
        .query_all(Statement::from_string(db.get_database_backend(), query))
        .await?;
    Ok(!result.is_empty())
}
