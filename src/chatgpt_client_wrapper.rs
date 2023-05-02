use crate::ModelConfigurationBuilder;
use crate::Result;
use crate::{ChatGPTEngine, CompletionResponse, Conversation, LibChatGPT, ModelConfiguration, Url};
use std::env;
use std::future::Future;
use std::path::Path;
use std::str::FromStr;

pub struct ChatGPTClientWrapper(pub LibChatGPT);

pub trait ChatGPTClient {
    fn new<S: Into<String>>(api_key: S) -> Result<ChatGPTClientWrapper>;
    fn new_with_config<S: Into<String>>(
        api_key: S,
        config: ModelConfiguration,
    ) -> Result<ChatGPTClientWrapper>;

    fn restore_conversation_postcard<P: AsRef<Path> + Send + Sync + 'static>(
        &self,
        file: P,
    ) -> Result<Box<dyn Future<Output = Result<Conversation>> + Send + '_>>;

    fn new_conversation(&self) -> Conversation;
    fn new_conversation_directed<S: Into<String>>(&self, direction_message: S) -> Conversation;
    fn send_message<S: Into<String> + Send + 'static>(
        &self,
        prompt: S,
    ) -> Box<dyn Future<Output = Result<CompletionResponse>> + Send + '_>;
    // fn send_message_streaming<S: Into<String>>(&self, message: S);
}
impl ChatGPTClient for ChatGPTClientWrapper {
    fn new<S: Into<String>>(api_key: S) -> Result<ChatGPTClientWrapper> {
        Ok(ChatGPTClientWrapper(LibChatGPT::new(api_key)?))
    }
    fn new_with_config<S: Into<String>>(
        api_key: S,
        config: ModelConfiguration,
    ) -> Result<ChatGPTClientWrapper> {
        Ok(ChatGPTClientWrapper(LibChatGPT::new_with_config(
            api_key, config,
        )?))
    }
    fn restore_conversation_postcard<P: AsRef<Path> + Send + Sync + 'static>(
        &self,
        file: P,
    ) -> Result<Box<dyn Future<Output = Result<Conversation>> + Send + '_>> {
        let fut = async move { self.0.restore_conversation_postcard(file).await };
        Ok(Box::new(fut))
    }
    fn new_conversation(&self) -> Conversation {
        self.0.new_conversation()
    }
    fn new_conversation_directed<S: Into<String>>(&self, direction_message: S) -> Conversation {
        self.0.new_conversation_directed(direction_message)
    }
    fn send_message<S: Into<String> + Send + 'static>(
        &self,
        prompt: S,
    ) -> Box<dyn Future<Output = Result<CompletionResponse>> + Send + '_> {
        let fut = async move { self.0.send_message(prompt).await };
        Box::new(fut)
    }
}
impl ChatGPTClientWrapper {
    fn load_api_key() -> String {
        // get api key or exit
        env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY environment variable")
    }
    fn default_client() -> Result<ChatGPTClientWrapper> {
        let client = LibChatGPT::new(Self::load_api_key())?;
        Ok(ChatGPTClientWrapper(client))
    }
    fn custom_client(
        api_url: Option<&str>,
        temperature: Option<f32>,
        engine: Option<ChatGPTEngine>,
    ) -> Result<ChatGPTClientWrapper> {
        let mut config_builder = ModelConfigurationBuilder::default();
        if let (None, None, None) = (api_url, temperature, engine) {
            return Self::default_client();
        }
        if let Some(url) = api_url {
            config_builder = config_builder.api_url(Url::from_str(url).unwrap()).clone();
        }

        if let Some(temp) = temperature {
            config_builder = config_builder.temperature(temp).clone();
        }

        if let Some(eng) = engine {
            config_builder = config_builder.engine(eng).clone();
        }

        let client =
            LibChatGPT::new_with_config(Self::load_api_key(), config_builder.build().unwrap())?;

        Ok(ChatGPTClientWrapper(client))
    }
}
