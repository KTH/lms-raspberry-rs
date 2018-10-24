extern crate env_logger;
#[macro_use]
extern crate log;
extern crate reqwest;
#[cfg(feature = "sensehat")]
extern crate sensehat;
extern crate sensehat_screen;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "sensehat")]
use sensehat::{Colour, SenseHat, SenseHatResult};

fn main() {
    env_logger::init();
    if let Err(e) = hello_hat() {
        eprintln!("Error on hat: {:?}", e);
    }
    let state = MonitorState::load();
    show_state(state).unwrap();
}

#[cfg(feature = "sensehat")]
fn hello_hat() -> SenseHatResult<()> {
    let mut hat = SenseHat::new()?;
    println!("Pressure: {:?}", hat.get_pressure());
    hat.text(" Hello from Rust!  ", Colour::RED, Colour::BLACK)?;
    Ok(())
}

#[cfg(not(feature = "sensehat"))]
fn hello_hat() -> Result<(), ()> {
    Ok(())
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

#[cfg(not(feature = "sensehat"))]
fn show_state(state: MonitorState) -> Result<(), ()> {
    eprintln!("Status is {:?}", state);
    Ok(())
}

#[cfg(feature = "sensehat")]
fn show_state(state: MonitorState) -> SenseHatResult<()> {
    eprintln!("Status is {:?}", state);
    let mut screen =
        sensehat_screen::Screen::open("/dev/fb1").map_err(|_| SenseHatError::ScreenError)?;
    let img = match state {
        MonitorState::Happy => PixelFrame::BLUE,
        MonitorState::Neutral => PixelFrame::BLACK,
        MonitorState::Sad => PixelFrame::RED,
        MonitorState::CheckFailed => PixelFrame::YELLOW,
    };
    screen.write_frame(&img.frame_line());
    Ok(())
}
