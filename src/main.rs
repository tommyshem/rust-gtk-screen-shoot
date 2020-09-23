use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "screen-shot",
    about = "Takes a screen, window or section of the screen shot and saves it to file or clipboard."
)]
struct Opt {
    /// Send the grab directly to the clipboard
    #[structopt(short = "c", long = "clipboard")]
    clipboard: bool,

    /// Grab a window instead of the entire screen
    #[structopt(short = "w", long = "window")]
    window: bool,

    /// Grab an area of the screen instead of the entire screen
    #[structopt(short = "a", long = "area")]
    area: bool,

    /// Include the pointer with the screenshot
    #[structopt(short = "p", long = "include-pointer")]
    pointer: bool,

    /// Take screenshot after specified delay [in seconds]
    #[structopt(short = "d", long = "delay", default_value = "0")]
    delay: u32,

    /// Interactively set options
    #[structopt(short = "i", long = "interactive")]
    interactive: bool,

    /// file location to save the image capture
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
}
fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}

// Testing cli options
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_args_passed() {
        let opt = Opt::from_iter(&["test"]);
        assert_eq!(format!("{:?}",opt), "Opt { clipboard: false, window: false, area: false, pointer: false, delay: 0, interactive: false, output: None }");
    }
}
