extern crate env_logger;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate sensehat_screen;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use sensehat_screen::{PixelColor, PixelFrame};
use sensehat_screen::error::ScreenError;
use sensehat_screen::frame::rotate::Rotate;
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;

fn main() {
    env_logger::init();
    let state = Arc::new(Mutex::new(MonitorState::Unknown));
    let readstate = state.clone();
    thread::spawn(move || -> () {
        let interval = Duration::from_millis(125);
        for i in (0..12).cycle() {
            let state: MonitorState = readstate.lock().unwrap().clone();
            show_state(state, i)
                .map_err(|e| {
                    error!("Failed to show state: {:?}", e);
                })
                .ok();
            sleep(interval);
        }
    });
    loop {
        *state.lock().unwrap() = MonitorState::load();
        sleep(Duration::from_millis(1111));
    }
}

#[derive(Debug, Clone)]
enum MonitorState {
    Happy,
    Neutral,
    Sad,
    Unknown,
}

#[derive(Debug, Deserialize)]
struct SrcStatus {
    name: String,
    color: String,
}

impl MonitorState {
    fn load() -> Self {
        reqwest::get("https://app-r.referens.sys.kth.se/lms-monitor-of-monitor/api")
            .and_then(|mut resp| resp.json())
            .map(|statuses: Vec<SrcStatus>| {
                debug!("Statuses is: {:?}", statuses);
                if statuses.iter().any(|s| s.color == "red") {
                    MonitorState::Sad
                } else if statuses.iter().all(|s| s.color == "blue") {
                    MonitorState::Happy
                } else {
                    MonitorState::Neutral
                }
            })
            .map_err(|e| {
                warn!("Failed to get status: {:?}", e);
            })
            .unwrap_or(MonitorState::Unknown)
    }
}

lazy_static! {
    static ref HAPPY_FRAMES: [PixelFrame; 3] = {
        let n = PixelColor::BLACK;
        let b = PixelColor::BLUE;
        let r = PixelColor::RED;
        [
            PixelFrame::from([
                n, n, n, n, n, n, n, n,
                n, b, n, n, n, n, b, n,
                n, n, n, n, n, n, n, n,
                n, n, n, n, n, n, n, n,
                r, n, n, n, n, n, n, r,
                n, r, n, n, n, n, r, n,
                n, n, r, r, r, r, n, n,
                n, n, n, n, n, n, n, n,
            ]).rotate(Rotate::Ccw180),
            PixelFrame::from([
                n, n, n, n, n, n, n, n,
                n, n, n, n, n, n, b, n,
                n, n, n, n, n, n, n, n,
                n, n, n, n, n, n, n, n,
                r, n, n, n, n, n, n, r,
                n, r, n, n, n, n, r, n,
                n, n, r, r, r, r, n, n,
                n, n, n, n, n, n, n, n,
            ]).rotate(Rotate::Ccw180),
            PixelFrame::from([
                n, n, n, n, n, n, n, n,
                n, b, n, n, n, n, n, n,
                n, n, n, n, n, n, n, n,
                n, n, n, n, n, n, n, n,
                r, n, n, n, n, n, n, r,
                n, r, n, n, n, n, r, n,
                n, n, r, r, r, r, n, n,
                n, n, n, n, n, n, n, n,
            ]).rotate(Rotate::Ccw180),
        ]
    };
    static ref NEUTRAL_FRAMES: [PixelFrame; 2] = {
        let n = PixelColor::BLACK;
        let b = PixelColor::BLUE;
        let r = PixelColor::RED;
        [
            PixelFrame::from([
                n, n, n, n, n, n, n, n,
                n, b, n, n, n, b, n, n,
                n, n, n, n, n, n, n, n,
                n, n, n, n, n, n, n, n,
                n, n, n, n, n, n, n, n,
                n, r, r, r, r, r, r, n,
                n, n, n, n, n, n, n, n,
                n, n, n, n, n, n, n, n,
            ]).rotate(Rotate::Ccw180),
            PixelFrame::from([
                n, n, n, n, n, n, n, n,
                n, n, b, n, n, n, b, n,
                n, n, n, n, n, n, n, n,
                n, n, n, n, n, n, n, n,
                n, n, n, n, n, n, n, n,
                n, r, r, r, r, r, r, n,
                n, n, n, n, n, n, n, n,
                n, n, n, n, n, n, n, n,
            ]).rotate(Rotate::Ccw180),
        ]
    };
    static ref SAD_FRAMES: [PixelFrame; 4] = {
        let n = PixelColor::BLACK;
        let b = PixelColor::BLUE;
        let r = PixelColor::RED;
        [
            PixelFrame::from([
                n, n, n, n, n, n, n, n,
                n, b, n, n, n, b, n, n,
                n, n, n, n, n, n, n, n,
                n, n, n, n, n, n, n, n,
                n, n, r, r, r, r, n, n,
                n, r, n, n, n, n, r, n,
                r, n, n, n, n, n, n, r,
                n, n, n, n, n, n, n, n,
            ]).rotate(Rotate::Ccw180),
            PixelFrame::from([
                n, n, n, n, n, n, n, n,
                n, n, b, n, n, n, b, n,
                n, n, n, n, n, n, n, n,
                n, n, n, n, n, n, n, n,
                n, n, r, r, r, r, n, n,
                n, r, n, n, n, n, r, n,
                r, n, n, n, n, n, n, r,
                n, n, n, n, n, n, n, n,
            ]).rotate(Rotate::Ccw180),
            PixelFrame::from([
                n, n, n, n, n, n, n, n,
                n, n, n, n, n, n, n, n,
                n, n, b, n, n, n, b, n,
                n, n, n, n, n, n, n, n,
                n, n, r, r, r, r, n, n,
                n, r, n, n, n, n, r, n,
                r, n, n, n, n, n, n, r,
                n, n, n, n, n, n, n, n,
            ]).rotate(Rotate::Ccw180),
            PixelFrame::from([
                n, n, n, n, n, n, n, n,
                n, n, n, n, n, n, n, n,
                n, b, n, n, n, b, n, n,
                n, n, n, n, n, n, n, n,
                n, n, r, r, r, r, n, n,
                n, r, n, n, n, n, r, n,
                r, n, n, n, n, n, n, r,
                n, n, n, n, n, n, n, n,
            ]).rotate(Rotate::Ccw180),
        ]
    };
}

fn show_state(state: MonitorState, i: usize) -> Result<(), ScreenError> {
    debug!("Status is {:?} (for {})", state, i);
    let mut screen = sensehat_screen::Screen::open("/dev/fb1")?;
    let frames: &[PixelFrame] = match state {
        MonitorState::Happy => HAPPY_FRAMES.as_ref(),
        MonitorState::Neutral => NEUTRAL_FRAMES.as_ref(),
        MonitorState::Sad => SAD_FRAMES.as_ref(),
        MonitorState::Unknown => NEUTRAL_FRAMES.as_ref(),
    };
    let img = frames[i % frames.len()];
    screen.write_frame(&img.frame_line());
    Ok(())
}
