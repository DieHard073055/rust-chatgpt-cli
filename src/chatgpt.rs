use crate::chatgpt_client_wrapper::{ChatGPTClient, ChatGPTClientWrapper};
use crate::debug_println;

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
