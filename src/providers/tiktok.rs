use crate::providers::Provider;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::builder::CreateMessage;

pub struct TiktokProvider;

lazy_static! {
    static ref RE_TIKTOK: Regex = Regex::new(r"(?:(?:www|vm)\.)?tiktok\.com\S+").unwrap();
}

impl Provider for TiktokProvider {
    fn name(&self) -> String {
        "tiktok".to_string()
    }
    fn new_message(&self, url: &str) -> CreateMessage {
        let mut message = CreateMessage::default();
        message.content(format!("https://tiktok.sauce.sh/?url=https://{url}"));
        message
    }

    fn match_url<'a>(&self, text: &'a str) -> Option<&'a str> {
        match RE_TIKTOK.find(text) {
            Some(m) => Some(m.as_str()),
            None => None,
        }
    }
}
