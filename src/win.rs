use std::{fmt::Display, thread};

use super::*;

use native_windows_gui::{MessageButtons, MessageIcons, MessageParams};

pub fn log_and_notify(msg: impl Display, level: log::Level, threaded: bool, delay_millis: u64) {
    let icons: MessageIcons;
    match level {
        Level::Error => {
            icons = MessageIcons::Error;
            error!("{}", msg);
        }
        Level::Warn => {
            icons = MessageIcons::Warning;
            warn!("{}", msg);
        }
        Level::Info => {
            icons = MessageIcons::Info;
            info!("{}", msg);
        }
        Level::Debug => {
            icons = MessageIcons::None;
            debug!("{}", msg);
        }
        Level::Trace => {
            icons = MessageIcons::None;
            trace!("{}", msg);
        }
    };
    let content = msg.to_string();

    if threaded {
        thread::spawn(move || {
            if delay_millis != 0 {
                sleep(Duration::from_millis(delay_millis));
            }
            let params = MessageParams {
                title: "Clan Tag Gif",
                content: content.as_str(),
                buttons: MessageButtons::Ok,
                icons,
            };
            native_windows_gui::message(&params)
        });
    } else {
        if delay_millis != 0 {
            sleep(Duration::from_millis(delay_millis));
        }
        let params = MessageParams {
            title: "Clan Tag Gif",
            content: content.as_str(),
            buttons: MessageButtons::Ok,
            icons,
        };
        native_windows_gui::message(&params);
    }
}
