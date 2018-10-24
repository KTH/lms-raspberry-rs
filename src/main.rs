extern crate env_logger;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate sensehat_screen;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use sensehat_screen::{PixelColor, PixelFrame};
use sensehat_screen::error::ScreenError;

fn main() {
    env_logger::init();
    let state = MonitorState::load();
    show_state(state).unwrap();
}

#[derive(Debug)]
enum MonitorState {
    Happy,
    Neutral,
    Sad,
    CheckFailed,
}

#[derive(Debug, Deserialize)]
struct SrcStatus {
    name: String,
    color: String,
}

impl MonitorState {
    fn load() -> Self {
        let mut resp = reqwest::get("https://app-r.referens.sys.kth.se/lms-monitor-of-monitor/api").unwrap();
        if resp.status().is_success() {
            let statuses : Vec<SrcStatus> = resp.json().unwrap();
            debug!("Statuses is: {:?}", statuses);
            if statuses.iter().any(|s| s.color == "red") {
                MonitorState::Sad
            } else if statuses.iter().all(|s| s.color == "blue") {
                MonitorState::Happy
            } else {
                MonitorState::Neutral
            }
        } else {
            MonitorState::CheckFailed
        }
    }
}

fn show_state(state: MonitorState) -> Result<(), ScreenError> {
    eprintln!("Status is {:?}", state);
    let mut screen =
        sensehat_screen::Screen::open("/dev/fb1")?;
    let n = PixelColor::BLACK;
    let b = PixelColor::BLUE;
    let r = PixelColor::RED;
    let img = match state {
        MonitorState::Happy => PixelFrame::from([
            n, n, n, n, n, n, n, n,
            n, b, n, n, n, n, b, n,
            n, n, n, n, n, n, n, n,
            n, n, n, n, n, n, n, n,
            r, n, n, n, n, n, n, r,
            n, r, n, n, n, n, r, n,
            n, n, r, r, r, r, n, n,
            n, n, n, n, n, n, n, n,
        ]),
        MonitorState::Neutral => PixelFrame::BLACK,
        MonitorState::Sad => PixelFrame::RED,
        MonitorState::CheckFailed => PixelFrame::YELLOW,
    };
    screen.write_frame(&img.frame_line());
    Ok(())
}
