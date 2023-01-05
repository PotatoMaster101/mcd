//! Small tool to automatically press CTRL and left click in random intervals.
#![forbid(missing_debug_implementations, missing_docs, unsafe_code)]

use std::thread::sleep;
use std::time::Duration;
use clap::Parser;
use inputbot::{KeybdKey::*, MouseButton::*, handle_input_events};
use rand::Rng;

/// Minimal interval in milliseconds
const MIN_INTERVAL: u32 = 75;

/// CLI arguments options.
#[derive(Parser, Debug)]
#[clap(about = "Automatically press CTRL and left click in random intervals, use CAPS-LOCK to toggle")]
struct Cli {
    #[clap(short, long, display_order = 1, help = "Milliseconds for maximum wait time, defaults to 75")]
    pub interval: Option<u32>,

    #[clap(short, long, display_order = 2, help = "Sneak key to press, defaults to CTRL")]
    pub key: Option<String>,

    #[clap(long, display_order = 2, help = "Do not use mouse click")]
    pub noclick: bool
}

fn main() {
    let args = Cli::parse();
    let interval = match args.interval {
        None => MIN_INTERVAL + 1,
        Some(i) => if i <= MIN_INTERVAL { MIN_INTERVAL + 1 } else { i }
    };
    let key = match args.key {
        None => LControlKey,
        Some(i) => if i == "SHIFT" { LShiftKey } else { LControlKey }
    };
    let noclick = args.noclick;

    CapsLockKey.bind(move || {
        let mut rng = rand::thread_rng();
        while CapsLockKey.is_toggled() {
            sleep(Duration::from_millis(rng.gen_range(MIN_INTERVAL..interval).into()));
            key.press();
            if !noclick {
                LeftButton.press();
            }
            sleep(Duration::from_millis(MIN_INTERVAL.into()));
            key.release();
            if !noclick {
                LeftButton.release();
            }
        }
    });
    println!("Use CAPS-LOCK to toggle");
    handle_input_events();
}
