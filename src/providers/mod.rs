use serenity::builder::CreateMessage;

pub mod tiktok;

pub trait Provider: Sync {
    fn match_url<'a>(&self, text: &'a str) -> Option<&'a str>;
    fn new_message(&self, url: &str) -> CreateMessage;
}
