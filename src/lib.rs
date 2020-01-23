use dbus::{arg, blocking::Connection};
use std::collections::HashMap;
use std::time::Duration;

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

pub fn get_cmus(artist: &mut String, title: &mut String) {
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
