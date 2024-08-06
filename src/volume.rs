use std::process::Command;

fn is_muted() -> bool {
    let output = Command::new("pamixer").arg("--get-mute").output().expect("pamixer couldn't be run");
    let status = String::from_utf8(output.stdout).expect("Couldn't capture the pamixer output."); 
    let status = status.trim().to_string(); 

    if status == "true" {
        return true;
    } else {
        return false;
    }
}

/* Returns the volume in percent */
fn get_volume() -> i8 {
    let output = Command::new("pamixer").arg("--get-volume").output().expect("pamixer couldn't be run.");
    let volume = String::from_utf8(output.stdout).expect("Couldn't capture the pamixer output."); 
    let volume = volume.trim().to_string();
    let mut volume = volume.parse::<i16>().unwrap();

    /* Returns 100% if set above 100% */
    if volume > 100 {
        volume = 100
    }

    /* returning an i8 value  */
    volume.try_into().unwrap()
}

/* Draws the volume bar */
pub fn draw_bar() -> String {
    /* dividing the volume (in percent) by ten to
     * get the number of full bits in a bar */
    let level = get_volume() / 10;
    /* Prefix the bar with a speaker icon */
    let mut bar: String;
    if !is_muted() {
        bar = String::from(" ");
    } else {
        bar = String::from(" ");
    }

    let mut iterator: i8 = 0;

    /* Adding the full bits to the bar */
    while iterator < level {
        bar += "ﭳ";
        iterator += 1;
    }

    /* Adding the empty bits to the bar */
    while iterator < 10 {
        bar += "—";
        iterator += 1;
    }

    bar
}
