use std::io::{self, Write};
use std::process;

use serde::{Deserialize, Serialize};

use dbus::{arg, blocking::Connection};
use std::collections::HashMap;
use std::time::Duration;

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

fn ret_refarg(value: &dyn arg::RefArg) -> &str {
    let mut ret = "Unknown";
    if let Some(string) = value.as_str() {
        ret = string;
    }
    // why
    else if let Some(ite) = value.as_iter() {
        for a in ite {
            if let Some(a_i) = a.as_iter() {
                for ab in a_i {
                    if let Some(a_s) = ab.as_str() {
                        ret = a_s;
                    }
                }
            }
        }
    }
    ret
}

fn dbus_m(artist: &mut String, title: &mut String) {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(
        "org.mpris.MediaPlayer2.cmus",
        "/org/mpris/MediaPlayer2",
        Duration::from_millis(5000),
    );
    use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
    let a: HashMap<String, arg::Variant<Box<dyn arg::RefArg>>> = proxy
        .get("org.mpris.MediaPlayer2.Player", "Metadata")
        .unwrap();
    for (k, v) in a {
        if k == "xesam:artist" {
            *artist = ret_refarg(&v).to_string();
        }
        if k == "xesam:title" {
            *title = ret_refarg(&v).to_string();
        }
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
        dbus_m(&mut artist, &mut title);

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
