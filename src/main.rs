use atty::Stream;
use clap::{App, Arg, ArgMatches};
use rust_chatgpt_cli::chatgpt::ChatGPT;
use std::io::{self, Read};

fn check_stdin() -> String {
    let mut stdin_prompt = String::new();
    // Check if stdin has data (e.g., "cat some-file | chatgpt")
    if !atty::is(Stream::Stdin) {
        let mut stdin_contents = String::new();
        let mut stdin = io::stdin();
        let stdin_result = stdin.read_to_string(&mut stdin_contents);

        if stdin_result.is_ok() {
            let contents = stdin_contents.trim();
            if !contents.is_empty() {
                println!("Handling prompt from stdin: {}", contents);
                stdin_prompt = contents.to_string();
            }
        }
    }
    stdin_prompt
}

fn configure_application() -> ArgMatches {
    App::new("chatgpt")
        .about("A chatbot application")
        .arg(
            Arg::with_name("prompt")
                .help("Enter your prompt")
                .multiple_values(true)
                .index(1)
                .required(false),
        )
        .arg(
            Arg::with_name("new-conversation")
                .short('n')
                .long("new-conversation")
                .help("Start a new conversation")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("list")
                .short('l')
                .long("list")
                .help("List all conversations")
                .takes_value(false)
                .conflicts_with_all(&["prompt", "new-conversation", "del"]),
        )
        .arg(
            Arg::with_name("del")
                .short('d')
                .long("del")
                .help("Delete a convo by INDEX")
                .takes_value(true)
                .value_name("INDEX")
                .conflicts_with_all(&["prompt", "new-conversation", "list"]),
        )
        .get_matches()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin_prompt = check_stdin();

    let matches = configure_application();

    // Handle the different argument scenarios
    if matches.is_present("list") {
        // List all conversations
        println!("Listing conversations...");
    } else if matches.is_present("del") {
        // Delete a conversation by index
        let index = matches.value_of("del").expect("Missing conversation index");
        println!("Deleting conversation with index {}...", index);
    } else {
        // Process the chatbot prompt
        let prompt = if stdin_prompt.is_empty() {
            matches
                .values_of("prompt")
                .map(|prompt_values| prompt_values.collect::<Vec<_>>().join(" "))
                .unwrap_or_else(|| String::new())
        } else {
            stdin_prompt
        };

        let new_conversation = matches.is_present("new-conversation");
        println!(
            "Processing prompt: '{}', new conversation: {}",
            &prompt, new_conversation
        );
        println!("{:}", ChatGPT::basic_query(&prompt).await?);
    }
    Ok(())
}
