> Do you even utilize this desk at all?

# posture_reminder 

Very simple reminder application for my Linux setup. Core idea stems from not utilizing the adjustable desk. This project is a solution to my own problem and was designed with `waybar` status bar in mind.

## Building

To build the project simply clone the git repository and run `cargo build --release`.
```bash
git clone https://github.com/qbibubi/posture_reminder.git
cd posture_reminder
cargo build --release
```

## Using built project

After build has completed move the `target/release/posture_reminder` into a directory of choice. Path to this binary will be executed by `waybar` in the config. Example use case:

```json
// waybar/config
{
    "modules-right": [ "custom/posture_reminder" ],

    // ...

    "custom/posture_reminder": {
        "exec": "/path/to/posture_reminder",
        "return-type": "json",
        "interval": 10,
        "on-click": "/path/to/posture_reminder --toggle"
    }
}
```

Feel free to tweak around the source code and styling. Enjoy code.

