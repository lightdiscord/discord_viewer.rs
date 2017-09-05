extern crate discord;
extern crate ansi_term;

use discord::Discord;

use ansi_term::Colour;

pub mod util;
pub mod handler;
pub mod connection;

fn main() {
  match util::get_token() {
    Some(token) => {
      match Discord::from_user_token(&token) {
        Ok(discord) => {
          connection::connect(discord)
        },
        Err(_) => {
          println!("{} Login failed.", Colour::Red.paint("error"));
        }
      }
    },
    None => {
      println!("{} Expected token.", Colour::Red.paint("error"));
    }
  }
}