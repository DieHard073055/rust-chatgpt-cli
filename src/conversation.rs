use chatgpt::prelude::ChatGPT;
use std::env;

#[tokio::main]
pub async fn basic_query(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    // get api key or exit
    let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY environment variable");

    // create a cilent
    let client = ChatGPT::new(api_key)?;

    // query chatgpt
    let response = client.send_message(prompt).await?;
    let content = &response.message().content;
    println!("Response: {:}", &content);

    Ok(content.clone())
}
