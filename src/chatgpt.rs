use crate::chatgpt_client_wrapper::{ChatGPTClient, ChatGPTClientWrapper};
use crate::debug_println;
use std::fs;
use std::path::Path;

const CONVERSATION_DIR: &str = "./.conversations";

pub struct ChatGPTConversation {}
impl ChatGPTConversation {
    fn ensure_conversations_dir_exists() {
        if !Path::new(CONVERSATION_DIR).exists() {
            fs::create_dir(CONVERSATION_DIR)
                .expect("failed to create a directory for conversations!");
        }
    }
    fn generate_name() -> String {
        // TODO: count the number of conversations
        let num = ChatGPTConversation::list_conversations().len();
        format!("[Conversation ({:})]", num).to_string()
    }
    fn list_conversations() -> Vec<String> {
        let mut conversations = vec![];

        ChatGPTConversation::ensure_conversations_dir_exists();

        let paths = fs::read_dir(CONVERSATION_DIR).unwrap();
        for path in paths {
            conversations.push(path.unwrap().path().display().to_string())
        }
        conversations
    }
}
pub struct ChatGPT {}
impl ChatGPT {
    pub async fn basic_query(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = ChatGPTClientWrapper::default_client()?;
        // query chatgpt
        let response_future = client.send_message(prompt.to_owned());
        let response = Box::pin(response_future).await?;
        let content = response.message_choices[0].message.content.clone();
        debug_println!("Response: {:}", content);

        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    fn create_test_file(path: &str) -> std::io::Result<()> {
        let mut file = File::create(path).unwrap();
        file.write_all(b"test conversation").unwrap();
        file.flush()?;
        Ok(())
    }

    #[test]
    fn test_generate_name() {
        let name1 = ChatGPTConversation::generate_name();
        // to ensure the second file gets the correct name
        let test_filename1 = format!("{}/{}", CONVERSATION_DIR, name1);
        create_test_file(&test_filename1).unwrap();

        let name2 = ChatGPTConversation::generate_name();
        assert_ne!(name1, name2, "Generated names should be unique");

        // clean up
        fs::remove_file(test_filename1).unwrap();
    }

    #[test]
    fn test_list_conversations() {
        // Create some test conversation files
        let test_conversation1 = format!("{}/test_conversation1", CONVERSATION_DIR);
        create_test_file(&test_conversation1).unwrap();
        let test_conversation2 = format!("{}/test_conversation2", CONVERSATION_DIR);
        create_test_file(&test_conversation2).unwrap();

        let conversations = ChatGPTConversation::list_conversations();

        // Check if the test conversation files are listed
        assert!(
            conversations.contains(&test_conversation1),
            "List should contain test_conversation1"
        );
        assert!(
            conversations.contains(&test_conversation2),
            "List should contain test_conversation2"
        );

        // Clean up the test conversation files
        fs::remove_file(test_conversation1).unwrap();
        fs::remove_file(test_conversation2).unwrap();
    }
}
