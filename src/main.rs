extern crate env_logger;

extern crate regex;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate ws;

use regex::Regex;
use std::process;
use ws::*;

struct Client {
    out: Sender,
}

impl Handler for Client {

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send("STATE?")
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        match msg {
            Message::Text(text) => self.on_text_message(&text),
            _ => Ok(()),
        }
    }

    fn on_error(&mut self, _: Error) {
        let result = StatusResult { connected: false, state: None };
        println!("{}", serde_json::to_string(&result).unwrap());
        process::exit(1);
    }
}

impl Client {
    fn on_text_message(&mut self, msg: &str) -> Result<()> {
        if msg.starts_with("STATE:") {
            self.on_state_message(msg)
        } else {
            Ok(())
        }
    }

    fn on_state_message(&mut self, msg: &str) -> Result<()> {
        let state = parse_state(&msg[6..]);
        let result = StatusResult { connected: true, state: Some(state) };
        println!("{}", serde_json::to_string(&result).unwrap());

        process::exit(0);
    }
}

#[derive(Serialize, Debug)]
struct StatusResult {
    connected: bool,
    state: Option<State>,
}

#[derive(Serialize, Debug)]
struct State {
    users: Option<i32>,
    max_users: Option<i32>,
    load: Option<f32>,
}

fn parse_state(msg: &str) -> State {
    // USR65/500|LOAD0.49|

    let user_regex = Regex::new(r"USR(?P<users>\d+)/(?P<max_users>\d+)").unwrap();
    let load_regex = Regex::new(r"LOAD(?P<load>\d+\.\d+)").unwrap();

    let mut state = State {
        users: None,
        max_users: None,
        load: None,
    };

    for part in msg.split('|') {
        if let Some(caps) = user_regex.captures(part) {
            state.users = caps["users"].parse().ok();
            state.max_users = caps["max_users"].parse().ok();

        } else if let Some(caps) = load_regex.captures(part) {
            state.load = caps["load"].parse().ok();
        }
    }

    state
}

fn main() {
    env_logger::init();

    connect("ws://glidertracker.de:3389/", |out| Client { out: out } ).unwrap()
}
