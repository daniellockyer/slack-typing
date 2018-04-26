extern crate slack;

use slack::{Event, EventHandler, RtmClient};

struct Handler;

impl EventHandler for Handler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        if let Event::UserTyping { channel, user: _ } = event {
            let _ = cli.sender().send_typing(&channel);
        }
    }

    fn on_close(&mut self, _: &RtmClient) {}

    fn on_connect(&mut self, _: &RtmClient) {}
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let api_key = match args.len() {
        0 | 1 => panic!("No API key provided!"),
        x => args[x - 1].clone(),
    };

    let mut handler = Handler;
    let r = RtmClient::login_and_run(&api_key, &mut handler);

    if let Err(e) = r {
        panic!("Error: {}", e);
    }
}
