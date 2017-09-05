use discord::{Discord, State};
use discord::model::{ChannelType};
use ansi_term::Colour;
use handler;

pub fn connect (discord: Discord) {
  match discord.connect() {
    Ok(values) => {
        let (mut connection, ready) = values;

        let mut state = State::new(ready);

        let channel_count: usize = state.servers().iter()
            .map(|server| server.channels.iter()
            .filter(|channel| channel.kind == ChannelType::Text)
            .count()
        ).fold(0, |v, s| v + s);
        println!("{} {} logging {} servers with {} text channels.", Colour::Green.paint("success"), state.user().username, state.servers().len(), channel_count);

        handler::handle_events(&mut connection, &mut state);
    },
    Err(_) => {
      println!("{} Connection failed.", Colour::Red.paint("error"));
    }
  }
}