use chatgpt::prelude::{ChatGPT, ResponseChunk};
use futures_util::StreamExt;
use std::env;
use std::io::{stdout, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get api key or exit
    let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY environment variable");

    // create a cilent
    let client = ChatGPT::new(api_key)?;

    // query chatgpt
    let stream = client
        .send_message_streaming("explain github to me like I am a five year old, under 10 words.")
        .await?;

    stream
        .for_each(|response_token| async move {
            match response_token {
                ResponseChunk::Content {
                    delta,
                    response_index: _,
                } => {
                    print!("{delta}");
                    stdout().lock().flush().unwrap();
                }
                _ => {}
            }
        })
        .await;

    Ok(())
}
