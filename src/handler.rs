use discord::{ChannelRef, State, Connection};
use discord::model::{Event};
use ansi_term::Colour;

pub fn handle_events(connection: &mut Connection, state: &mut State) {
  loop {
    let event = match connection.recv_event() {
      Ok(event) => event,
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
            println!("{} {} {}",
              Colour::RGB(212,93,121).paint(format!("{}", if message.author.id == state.user().id {"→"} else {"←"})),
              Colour::RGB(234,144,133).paint(format!("{}", group.name())),
              Colour::RGB(212,93,121).paint(format!("@{}", message.author.name))
            );
            println!("{}", Colour::RGB(233,226,208).paint(format!("↳ {}", message.content)));
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