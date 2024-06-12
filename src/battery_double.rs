use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/* Global variables containing the path to batteries */
static BAT1: &'static str = "/sys/class/power_supply/BAT0";
static BAT2: &'static str = "/sys/class/power_supply/BAT1"; /* Ignore if you have only one battery */

/* Global variables containing the file names */
static NOW:  &'static str = "energy_now";
static FULL: &'static str = "energy_full";
static STATUS: &'static str = "status";

fn get_energy(bat: &str, which: &str) -> i32 {
    let mut value = String::from("0"); /* To return if `bat` isn't connected */

    /* Opening and reading the file if the `bat` is connected */
    if Path::new(bat).exists() { /* e.g /sys/class/power_supply/BAT1/energy_now */
        let mut file = File::open(format!("{bat}/{which}")).expect("Unable to open file.");
        value = String::new();
        file.read_to_string(&mut value).expect("Unable to read file.");
    }
    
    /* return the value parsed into an i32 (divided by 1000 to remove the zeros) */
    value.trim().parse::<i32>().unwrap() / 1000
}

fn get_status(bat: &str) -> String {
    let mut value = String::from("Not Charging"); /* To return if `bat` isn't connected */

    /* Opening and reading the file */
    if Path::new(bat).exists() { /* e.g /sys/class/power_supply/BAT1/status */
        let mut file = File::open(format!("{bat}/{STATUS}")).expect("Unable to open file.");
        value = String::new();
        file.read_to_string(&mut value).expect("Unable to read file.");
    }

    /* return the value (without the whitespace characters) */
    value.trim().to_string()
}

fn get_battery_percentage() -> i8 {
    /* Get the sum of the energy stored in both batteries */
    let energy_now:  i32 = get_energy(BAT1, NOW)  + get_energy(BAT2, NOW);
    let energy_full: i32 = get_energy(BAT1, FULL) + get_energy(BAT2, FULL);

    /* Return the percentage of the energy stored in batteries */
    (energy_now * 100 / energy_full).try_into().unwrap()
}

fn draw_charge() -> &'static str {
    let percentage: i8 = get_battery_percentage();

    let level: i8 = percentage / 20;
    match level {
        5 => return " ", /* = 100% */
        4 => return " ", /* >= 80% */
        3 => return " ", /* >= 60% */
        2 => return " ", /* >= 40% */
        1 => return " ", /* >= 20% */
        0 => return " ", /* >=  0% */
        _ => return "~~"  /* Return a fallback value */
    }
}

fn draw_status() -> &'static str {
    let tmp_status = get_status(BAT1); let bat1_status: &str = String::as_str(&tmp_status);
    let tmp_status = get_status(BAT2); let bat2_status: &str = String::as_str(&tmp_status);
    let percentage: i8 = get_battery_percentage();

    
    if bat1_status == "Full" && bat2_status == "Full" {
        return "";                         /* Full */
    } else if bat1_status == "Charging" || bat2_status == "Charging" {
        if percentage >= 95 { return ""; } /* Charging and 'Full' */
        else                { return ""; } /* Charging            */
    } else if bat1_status == "Discharging" || bat2_status == "Discharging" {
        if percentage > 10  { return ""; } /* Discharging         */
        else                { return ""; } /* Discharging and low */
    } else if bat1_status == "Not charging" && bat2_status == "Not charging" && percentage >= 95 {
        return "";
    }

    /* Return a fallback value */
    "~"
}

pub fn draw_icon() -> String {
    format!("{} {}", draw_charge(), draw_status())
}
