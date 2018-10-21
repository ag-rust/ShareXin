#[macro_use]
extern crate clap;

extern crate clipboard;
extern crate egg_mode_text;
extern crate gdk;
extern crate glib;
extern crate gtk;
extern crate imgur as Imgur;
extern crate notify_rust;
extern crate open;
extern crate screenshot_rs;
extern crate time;
extern crate yaml_rust;

mod cmd;
mod dialog;
mod error;
mod image;
mod imgur;
mod language;
mod mastodon;
mod notification;
mod save;
mod twitter;

#[derive(Copy, Clone)]
pub enum ServiceKind {
    Twitter,
    Mastodon,
    Imgur,
}

#[derive(Copy, Clone, PartialEq)]
pub enum MessageKind {
    Image,
    Text,
}

fn main() {
    cmd::cmd();
}
