use pi_aier_lib::db::*;
use pi_aier_lib::models::*;
use sea_orm::entity::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_save_session() {
        let db = initialize_database("sqlite://data.db?mode=rwc")
            .await
            .unwrap();

        let a = message::ActiveModel {
            id: Set(0),
            session_uuid: Set(uuid::Uuid::new_v4().to_string()),
            order: Set(1),
            itype: Set(ienum::SessionType::LONG.to_string()),
            content: Set("test".to_string()),
        }
        .insert(&db)
        .await;

        match a {
            Ok(j) => {
                println!("{:?}", j)
            }
            Err(d) => {
                eprintln!("{}", d)
            }
        }

        db.close().await.unwrap();
    }
}
