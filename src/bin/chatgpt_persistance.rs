use std::env;
use std::{fs, io, path::Path};

use chatgpt::prelude::ChatGPT;

const CONVERSATION_DIR: &str = ".conversations";
const TEST_CONVERSATION_FILE: &str = ".conversations/test-conversation.bin";

fn handle_conversation_save_dir() -> io::Result<()> {
    if !Path::new(CONVERSATION_DIR).exists() {
        fs::create_dir(CONVERSATION_DIR)?;
    }
    if Path::new(TEST_CONVERSATION_FILE).exists() {
        fs::remove_file(TEST_CONVERSATION_FILE)?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get api key or exit
    let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY environment variable");

    // create a cilent
    let client = ChatGPT::new(api_key)?;

    // create a conversation
    let mut conversation = client.new_conversation();
    conversation
        .send_message("Explain technical analysis in 10 words")
        .await?;
    conversation
        .send_message("Please do the same for fundemental analysis")
        .await?;

    // persist the conversation
    handle_conversation_save_dir()?;
    conversation
        .save_history_postcard(&TEST_CONVERSATION_FILE)
        .await?;
    // get rid of conversation since we will create a new one
    drop(conversation);

    let mut new_conversation = client
        .restore_conversation_postcard(&TEST_CONVERSATION_FILE)
        .await?;
    let response = new_conversation
        .send_message("Now do the same for value investing")
        .await?;

    for message in &new_conversation.history {
        println!("{:?}: {:}", message.role, message.content);
    }

    Ok(())
}
