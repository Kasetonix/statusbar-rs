use std::process::ExitCode;
use std::io::stdout;
use std::io::Write;

/* Comment out all of the unneded modules 
 * here and in print macros in the main function */
/* If you're on desktop you probably don't want the brightness and battery modules */
mod caps_lock;
mod volume;
mod brightness;
mod battery_single;
// mod battery_double;
mod clock;

fn main() -> ExitCode {
    print!(" {} ", caps_lock::draw_icon());
    print!(" {} ", volume::draw_bar());
    print!("{}", brightness::draw_bar());
    print!(" {} ", battery_single::draw_icon());
    // print!(" {} ", battery_double::draw_icon());
    print!(" {} ", clock::draw_time());
    println!();
    stdout().flush().expect("Couldn't flush the stdout.");

    ExitCode::SUCCESS
}
