#![feature(str_split_as_str)]
use hyphenation::{Standard, Language};
use hyphenation::load::Error;
use futures::prelude::*;
use irc::client::prelude::*;
use std::collections::HashMap;
mod language_processor;

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    // load config
    let config = Config::load("butt.toml").unwrap();
    let mut client = Client::from_config(config).await?;

    // connect
    client.identify()?;
    let mut stream = client.stream()?;

    // initialize chat map
    //let mut chat_map: HashMap<String,String> = HashMap::new();

    // listen
    while let Some(message) = stream.next().await.transpose()? {
        if let Command::PRIVMSG(channel, message) = message.command {
            println!("{}: {}", channel, message);
            //chat_map.insert(client.current_nickname().to_string(), client.);
            //if message.contains(client.current_nickname()) {
            println!("{}", language_processor::get_buttified_sentence(message.clone(), 5));
            //    client.send_privmsg(&channel, language_processor::get_buttified_sentence(message)).unwrap();
            //}
            if message.starts_with("!g") {
                println!("https://duckduckgo.com/?q={}","query");
            }
        }
    }

    Ok(())
}

pub fn load_dict() -> Result<Standard, Error> {
    // alternatively, let path_to_dict = "en-us.standard.bincode";
    let en_us: Result<Standard, Error> = hyphenation::Load::from_embedded(Language::EnglishUS);
    return en_us;
}