use pi_aier_lib::*;

#[tokio::test]
async fn test_gpt() {
    let gpt_messages = vec![gpt::GptMessage {
        role: "user".to_string(),
        content: "test".to_string(),
    }];

    let config = gpt::get_config();
    let result = gpt::call_gpt(config.unwrap().models[0].clone(), &gpt_messages).await;

    println!("{:?}", result);
}
