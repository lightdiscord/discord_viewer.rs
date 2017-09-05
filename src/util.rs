use std::env;

pub fn get_token() -> Option<String> {
  let key = "DISCORD_TOKEN";
  match env::var(key) {
      Ok(value) => Some(value),
      Err(_) => None
  }
}