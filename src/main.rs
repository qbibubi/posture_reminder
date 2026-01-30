use chrono::Local;
use serde::{Serialize, Deserialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const SIT_MINUTES: i64 = 45;
const STAND_MINUTES: i64 = 15;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Mode {
    Sitting,
    Standing
}

#[derive(Serialize, Deserialize, Debug)]
struct State {
    mode: Mode,
    start_timestamp: i64,
}

#[derive(Serialize)]
struct StatusBarOutput {
    text: String,
    tooltip: String,
    class: String,
}

fn get_state_path() -> PathBuf {
    let mut path = dirs::cache_dir().unwrap_or(PathBuf::from("/tmp"));
    path.push("posture_state.json");
    path
}

fn save_state(state: &State) {
    let path = get_state_path();
    let data = serde_json::to_string(state).unwrap();
    let _ = fs::write(path, data);
}

fn load_state() -> State {
    let path = get_state_path();

    if let Ok(data) = fs::read_to_string(&path) {
        if let Ok(state) = serde_json::from_str(&data) {
            return state;
        }
    }

    State {
        mode: Mode::Sitting,
        start_timestamp: Local::now().timestamp(),
    }
}

fn send_notification(title: &str, body: &str) {
    let _ = Command::new("notify-send")
        .arg("-u").arg("critical")
        .arg(title)
        .arg(body)
        .spawn();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut state = load_state();

    if args.len() > 1 && args[1] == "--toggle" {
        state.mode = match state.mode {
            Mode::Sitting => Mode::Standing,
            Mode::Standing => Mode::Sitting
        };

        state.start_timestamp = Local::now().timestamp();
        
        save_state(&state);

        let message = match state.mode {
            Mode::Sitting => "Time to sit down (45 minutes)",
            Mode::Standing => "Time to stand up (15 minutes)",
        };

        send_notification("Posture changed", message);
        print_status(&state);
        return;
    }

    print_status(&state);
}

fn print_status(state: &State) {
    let now = Local::now().timestamp();
    let elapsed = now - state.start_timestamp;

    let (target_minutes, icon, label) = match state.mode {
        Mode::Sitting => (SIT_MINUTES, "🪑", "Sitting"), 
        Mode::Standing => (STAND_MINUTES, "🧍", "Standing"),
    };

    let target_seconds = target_minutes * 60;
    let remaining_seconds = target_seconds - elapsed;

    let mut class = String::from("normal");
    let text;
    
    if remaining_seconds > 0 {
        let minutes_left = remaining_seconds / 60;
        text = format!("{} {} minutes", icon, minutes_left);

        if minutes_left < 2 {
            class = String::from("warning");
        }
    }
    else {
        let overdue_minutes = (elapsed - target_seconds) / 60;
        text = format!("{} +{} minutes", icon, overdue_minutes);

        class = String::from("critical");

        if remaining_seconds == -1 || remaining_seconds == -2 {
            send_notification("Posture Alert", &format!("{} limit reached. Switch position now!", label));
        }
    }

    let output = StatusBarOutput {
        text,
        tooltip: format!("Current: {}\n Target: {} minutes\nElapsed: {} minutes", label, target_minutes, elapsed / 60),
        class: format!("{} {}", label.to_lowercase(), class),
    };

    println!("{}", serde_json::to_string(&output).unwrap());
}
