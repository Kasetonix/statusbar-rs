use std::process::Command;
use std::env;

pub fn draw_icon() -> String {
    /* Evaluated at compile time! */
    const HOME: &'static str = env!("HOME");
    let sb_path: String = format!("{HOME}/.config/statusbar-rs");
    let capsl_path: String = format!("{sb_path}/src/capsl.sh");

    let output = Command::new(capsl_path).output().expect("`capsl` couldn't be run.");
    let output = String::from_utf8(output.stdout).expect("Couldn't capture the `capsl` output."); 
    let mut icon = String::from(" ");
    
    if output.trim() == "true" {
        icon = "".to_string();
    }

    /* Returning the icon*/
    icon.to_string()
}
