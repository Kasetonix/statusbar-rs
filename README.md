# statusbar-rs

## Dependencies
* `brightnessctl` --- Getting the brightness values of the built-in screen
* `xorg-xset`, `grep`, `awk` --- Getting the state of caps lock
* `date` --- Getting the current time
* `pamixer` --- Getting the volume of the current pulseaudio sink

## Installation
`git clone https://github.com/Kasetonix/statusbar-rs`
`cd statusbar-rs`
`cargo build --release`

## Running
Many variables are (unfortunately) hard-coded (maybe I'll add a config file later). Fortunately I at least put them into consts at the top of the files, so they aren't really hidden. Here's a list of what one would probably need to change in order for the program to run:
* `src/battery_single.rs` **@ 5** --- `BAT` -> path to the directory containing the battery's status 
* `src/battery_single.rs` **@ 6** --- `CAPACITY` `STATUS` -> names of the files containing the state of the battery
* `src/battery_double.rs` **@ 6** --- `BAT1`, `BAT2` -> path to the directories containing the batteries' statuses
* `src/battery_double.rs` **@ 10** --- `NOW`, `FULL`, `STATUS` -> names of the files containing the states of the batteries
* `src/brightness.rs` **@ 6** --- `LID_STATE` -> path to the state file of the laptop lid
* `src/caps_lock.rs` **@ 7** --- `sb_path` -> path to this project

## "Configuration"
One can configure the displayed "widgets" by commenting in-and-out the correct modules (`src/main.rs` **@ 8**) and print! macros (`src/main.rs` **@ 16**).
