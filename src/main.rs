use std::{fs, io};
use std::io::Read;

use atty::Stream;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use toml;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    content: String,
    role: String,
}

#[derive(Serialize, Deserialize)]
struct Choice {
    index: i32,
    finish_reason: String,
    message: Message,
}

#[derive(Serialize, Deserialize)]
struct Usage {
    completion_tokens: i32,
    prompt_tokens: i32,
    total_tokens: i32,
}

#[derive(Serialize, Deserialize)]
struct Response {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    messages: Vec<Message>,
    temperature: f32,
    model: String,
    stream: bool,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Rust Sender", about = "Small program that sends data to Restful API.")]
struct Opt {
    /// File name to read from
    #[structopt(short = "f", long = "file")]
    file: Option<String>,

    /// Message to send
    #[structopt(short = "m", long = "message")]
    message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    url: String,
    key: String,
    prompt: String,
}

fn read_config() -> Config {
    let mut path = dirs::home_dir().unwrap();
    path.push(".haigpt");
    path.push("config.toml");

    let mut config_file = fs::File::open(&path)
        .expect("Unable to open config.toml");

    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content)
        .expect("Unable to read config.toml");

    let config: Config = toml::from_str(&config_content)
        .expect("Unable to parse config.toml");

    config
}

#[tokio::main]
async fn main() {
    let config = read_config();

    let opt = Opt::from_args();
    let mut input = String::new();
    let data = if !atty::is(Stream::Stdin) {
        io::stdin().read_to_string(&mut input).unwrap();
        input.trim().to_string()
    } else if let Some(file) = opt.file {
        let content = fs::read_to_string(file).unwrap();
        content
    } else if let Some(msg) = opt.message {
        msg
    } else {
        eprintln!("Please provide a file or a message");
        "hi".to_string()
        // panic!("No file or message provided")
    };

    let client = reqwest::Client::new();

    let data = Request {
        model: String::from("gpt-4"),
        temperature: 0.7,
        stream: false,
        messages: Vec::from([
            Message {
                role: String::from("system"),
                content: String::from(config.prompt),
            },
            Message {
                role: String::from("user"),
                content: String::from(data),
            },
        ]),
    };

    let res = client.post(config.url)
        .header("Authorization", format!("Bearer {}", config.key))
        .json(&data)
        .send()
        .await.unwrap();

    let json: Response = res.json().await.unwrap();
    println!("The response is: {}", json.choices[0].message.content);
}

