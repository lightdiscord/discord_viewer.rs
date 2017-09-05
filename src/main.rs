extern crate discord;
extern crate ansi_term;

use std::env;
use std::process::exit;

use discord::{Discord, ChannelRef, State, Connection};
use discord::model::{Event, ChannelType};

use ansi_term::Colour;

fn main() {
  match get_token() {
    Some(token) => {
      match Discord::from_user_token(&token) {
        Ok(discord) => {
          connection(discord)
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

fn connection (discord: Discord) {
  let (mut connection, ready) = match discord.connect() {
    Ok(val) => val,
    Err(_) => {
      println!("{} Connection failed.", Colour::Red.paint("error"));
      exit(1);
    }
  };

  let mut state = State::new(ready);

  let channel_count: usize = state.servers().iter()
    .map(|server| server.channels.iter()
      .filter(|channel| channel.kind == ChannelType::Text)
      .count()
    ).fold(0, |v, s| v + s);
  println!("{} {} logging {} servers with {} text channels", Colour::Green.paint("success"), state.user().username, state.servers().len(), channel_count);

  handle_events(&discord, &mut connection, &mut state);
}

fn get_token() -> Option<String> {
  let key = "DISCORD_TOKEN";
  match env::var(key) {
      Ok(value) => Some(value),
      Err(_) => None
  }
}

fn handle_events(discord: &Discord, connection: &mut Connection, state: &mut State) {
  loop {
    let event = match connection.recv_event() {
      Ok(event) => event,
      Err(discord::Error::Closed(code,_)) => {
        println!("{} Connection closed with status {:?}.", Colour::Red.paint("error"), code);
        break
      },
      Err(error) => {
        println!("{} Receive error: {:?}.", Colour::Red.paint("error"), error);
        continue
      }
    };
    state.update(&event);


    match event {
      Event::MessageCreate(message) => {
        match state.find_channel(message.channel_id) {
          Some(ChannelRef::Public(server, channel)) => {
            println!("{} {} {} {}",
              Colour::RGB(212,93,121).paint(format!("{}", if message.author.id == state.user().id {"→"} else {"←"})),
              Colour::RGB(234,144,133).paint(format!("{}", server.name)),
              Colour::RGB(110,87,115).paint(format!("#{}", channel.name)),
              Colour::RGB(212,93,121).paint(format!("@{}", message.author.name))
            );
            println!("{}", Colour::RGB(233,226,208).paint(format!("↳ {}", message.content)));
          },
          Some(ChannelRef::Group(group)) => {
            println!("[&{}] {}: {}", group.name(), message.author.name, message.content);
          },
          Some(ChannelRef::Private(channel)) => {
            if message.author.id == channel.recipient.id {
              println!("[@{}] ← {}", channel.recipient.name, message.content);
            } else {
              println!("[@{}] → {}", channel.recipient.name, message.content);
            }
          },
          None => {}
        }
      },
      _ => {}
    }
  }
}