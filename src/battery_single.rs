use std::fs::File;
use std::io::prelude::*;

/* Global variables containing the path to a battery */
static BAT: &'static str = "/sys/class/power_supply/BAT1";
static CAPACITY: &'static str = "capacity"; 
static STATUS: &'static str = "status";

fn get_percentage() -> i8 {
    /* Opening and reading the file */
    /* e.g /sys/class/power_supply/BAT1/capacity */
    let mut file = File::open(format!("{BAT}/{CAPACITY}")).expect("Unable to open file.");
    let mut value = String::new();
    file.read_to_string(&mut value).expect("Unable to read file.");
    
    /* return the value parsed into an i32 */
    value.trim().parse::<i8>().unwrap()
}

fn get_status() -> String {
    /* Opening and reading the file */
    /* e.g /sys/class/power_supply/BAT1/status */
    let mut file = File::open(format!("{BAT}/{STATUS}")).expect("Unable to open file.");
    let mut value = String::new();
    file.read_to_string(&mut value).expect("Unable to read file.");

    /* return the value (without the whitespace characters) */
    value.trim().to_string()
}

fn draw_charge() -> &'static str {
    /* Get the percentage from the get_energy function */
    let percentage: i8 = get_percentage(); 

    if percentage >= 95 {
        return " "; 
    }

    let level: i8 = percentage / 25;
    match level {
        3 => return " ", /* >= 75% */
        2 => return " ", /* >= 50% */
        1 => return " ", /* >= 25% */
        0 => return " ", /* >=  0% */
        _ => return "~~"  /* Return a fallback value */
    }
}

fn draw_status() -> &'static str {
    let tmp_status = get_status(); let bat_status: &str = String::as_str(&tmp_status);
    let percentage: i8 = get_percentage();
    
    if bat_status == "Full" {
        return "";                         /* Full */
    } else if bat_status == "Charging" {
        if percentage >= 99 { return ""; } /* Charging and 'Full' */
        else                { return ""; } /* Charging            */
    } else if bat_status == "Discharging" {
        if percentage > 10  { return ""; } /* Discharging         */
        else                { return ""; } /* Discharging and low */
    }

    /* Return a fallback value */
    "~"
}

pub fn draw_icon() -> String {
    format!("{} {}", draw_charge(), draw_status())
}