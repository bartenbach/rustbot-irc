use hyphenation::{Standard, Language};
use hyphenation::load::Error;
use futures::prelude::*;
use irc::client::prelude::*;
mod language_processor;

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    let config = Config::load("butt.toml").unwrap();
    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;

    while let Some(message) = stream.next().await.transpose()? {
        if let Command::PRIVMSG(channel, message) = message.command {
            if message.contains(client.current_nickname()) {
                client.send_privmsg(&channel, "hey there!").unwrap();
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