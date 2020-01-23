use std::io::{self, Write};
use std::process;

use serde::{Deserialize, Serialize};

use i3bar_cmus::get_cmus;

#[derive(Serialize, Deserialize)]
struct BarMessage<'a> {
    name: &'a str,
    markup: &'a str,
    full_text: &'a str,
}

fn print_line(line: String) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(line.as_bytes()).unwrap();
}

fn read_line(line: &mut String) {
    match io::stdin().read_line(line) {
        Ok(_n) => {}
        Err(_error) => process::exit(1),
    }
}

fn main() {
    let mut input = String::new();
    read_line(&mut input);
    print_line(input);
    input = String::new();
    read_line(&mut input);
    print_line(input);

    loop {
        let mut message = String::new();
        read_line(&mut message);

        let mut artist = String::new();
        let mut title = String::new();
        get_cmus(&mut artist, &mut title);

        let m = BarMessage {
            name: "test",
            markup: "none",
            full_text: &format!(" {} : {} ", artist, title),
        };

        message.pop();
        message.pop();
        message.push(',');
        message.push_str(&serde_json::to_string(&m).unwrap());
        message.push(']');
        message.push('\n');

        print_line(message)
    }
}
