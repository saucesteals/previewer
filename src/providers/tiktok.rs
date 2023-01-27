use crate::providers::Provider;
use lazy_static::lazy_static;
use rand::prelude::*;
use regex::Regex;
use serenity::builder::CreateMessage;

pub struct TiktokProvider;

lazy_static! {
    static ref RE_TIKTOK: Regex = Regex::new(r"(?:(?:www|vm)\.)?tiktok\.com\S+").unwrap();
}

const HIDER: &str = "||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||||​||";
const BUYMECOFFEE: &str = "https://www.buymeacoffee.com/saucesteals";

fn buy_me_coffee<'a>() -> String {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0..100) < 25 {
        return BUYMECOFFEE.to_string();
    }

    return format!("<{BUYMECOFFEE}>");
}

impl Provider for TiktokProvider {
    fn name(&self) -> String {
        "tiktok".to_string()
    }
    fn new_message(&self, url: &str) -> CreateMessage {
        let mut message = CreateMessage::default();
        message.content(format!(
            "{}\n{}\nhttps://tiktok.sauce.sh/?url=https://{}",
            buy_me_coffee(),
            HIDER,
            url
        ));
        message
    }

    fn match_url<'a>(&self, text: &'a str) -> Option<&'a str> {
        match RE_TIKTOK.find(text) {
            Some(m) => Some(m.as_str()),
            None => None,
        }
    }
}
