use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "screen-shot",
    about = "Takes a screen, window or section of the screen shot and saves it to file or clipboard."
)]
pub struct Opt {
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

// get arguments which are passed in from the command
pub fn get_arguments_passed_in() -> Opt {
    Opt::from_args()
}

#[test]
fn no_args() {
    let opt = Opt::from_iter(&["test"]);
    assert_eq!(format!("{:?}",opt), "Opt { clipboard: false, window: false, area: false, pointer: false, delay: 0, interactive: false, output: None }");
}

#[test]
fn one_flag_args() {
    // Tests below
    struct FlagTestCase {
        flag: String,
        result: String,
    }
    let one_flag_args_test = vec![
        FlagTestCase{flag: String::from("-c"),result: String::from("Opt { clipboard: true, window: false, area: false, pointer: false, delay: 0, interactive: false, output: None }")},
        FlagTestCase{flag: String::from("--clipboard"),result: String::from("Opt { clipboard: true, window: false, area: false, pointer: false, delay: 0, interactive: false, output: None }")},
        FlagTestCase{flag: String::from("-w"),result: String::from("Opt { clipboard: false, window: true, area: false, pointer: false, delay: 0, interactive: false, output: None }")},
        FlagTestCase{flag: String::from("--window"),result: String::from("Opt { clipboard: false, window: true, area: false, pointer: false, delay: 0, interactive: false, output: None }")},
        FlagTestCase{flag: String::from("-a"),result: String::from("Opt { clipboard: false, window: false, area: true, pointer: false, delay: 0, interactive: false, output: None }")},
        FlagTestCase{flag: String::from("--area"),result: String::from("Opt { clipboard: false, window: false, area: true, pointer: false, delay: 0, interactive: false, output: None }")},
        FlagTestCase{flag: String::from("-p"),result: String::from("Opt { clipboard: false, window: false, area: false, pointer: true, delay: 0, interactive: false, output: None }")},
        FlagTestCase{flag: String::from("--include-pointer"),result: String::from("Opt { clipboard: false, window: false, area: false, pointer: true, delay: 0, interactive: false, output: None }")},
        FlagTestCase{flag: String::from("-d"),result: String::from("Opt { clipboard: false, window: false, area: false, pointer: false, delay: 0, interactive: false, output: None }")},
        FlagTestCase{flag: String::from("--delay"),result: String::from("Opt { clipboard: false, window: false, area: false, pointer: false, delay: 0, interactive: false, output: None }")},
        FlagTestCase{flag: String::from("-i"),result: String::from("Opt { clipboard: false, window: false, area: false, pointer: false, delay: 0, interactive: true, output: None }")},
        FlagTestCase{flag: String::from("--interactive"),result: String::from("Opt { clipboard: false, window: false, area: false, pointer: false, delay: 0, interactive: true, output: None }")},
        ];
    // run through tests
    for (_index, value) in one_flag_args_test.iter().enumerate() {
        let opt = Opt::from_iter(&["test", &value.flag]);
        assert_eq!(format!("{:?}", opt), value.result);
    }
}
