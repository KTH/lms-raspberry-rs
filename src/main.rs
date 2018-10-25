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

use sensehat_screen::error::ScreenError;
use sensehat_screen::frame::rotate::Rotate;
use sensehat_screen::{PixelColor, PixelFrame};
use std::sync::Mutex;
use std::thread::{self, sleep};
use std::time::Duration;

fn main() {
    env_logger::init();
    let check_interval = Duration::from_millis(3456);
    let frame_interval = Duration::from_millis(234);

    thread::spawn(move || -> () {
        for i in (0..12).cycle() {
            let state = get_state();
            show_state(state, i)
                .map_err(|e| {
                    error!("Failed to show state: {:?}", e);
                })
                .ok();
            sleep(frame_interval);
        }
    });
    loop {
        set_state(MonitorState::load());
        sleep(check_interval);
    }
}

lazy_static! {
    static ref CURRENT_STATE: Mutex<MonitorState> = Mutex::new(MonitorState::Unknown);
}

fn get_state() -> MonitorState {
    CURRENT_STATE
        .lock()
        .map(|s| s.clone())
        .unwrap_or(MonitorState::Unknown)
}

fn set_state(state: MonitorState) {
    if let Ok(mut current_state) = CURRENT_STATE.lock() {
        *current_state = state;
    } else {
        error!("Failed to get lock on current state");
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
