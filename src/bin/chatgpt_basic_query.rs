use std::env;

use chatgpt::prelude::ChatGPT;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get api key or exit
    let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY environment variable");

    // create a cilent
    let client = ChatGPT::new(api_key)?;

    // query chatgpt
    let response = client
        .send_message("whats the meaning of life? answer with just a number.")
        .await?;
    println!("Response: {:}", response.message().content);

    Ok(())
}
