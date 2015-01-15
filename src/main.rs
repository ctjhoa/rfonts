//extern crate curl;
extern crate cli;
extern crate getopts;

use std::os;

fn main() {
    let opts = &[
        cli::helpopt(),
        cli::versionopt(),
        getopts::optopt("o", "", "Set output file name", "FILENAME"),
    ];

    let matches = cli::parse_args(opts);
    if matches.opt_present("h") {
        println!("{}", cli::usage_string(opts));
        return;
    }
    if matches.opt_present("version") {
        println!("{}", cli::version_string("0.0.1"));
        return;
    }

}

//use StringResult::StringOK;
//use StringResult::ErrorReason;
//
//
//enum StringResult {
//    StringOK(String),
//    ErrorReason(String),
//}
//
//fn main() {
//    println!("Hello, world!");
//    match get_font_dir("tta") {
//        ErrorReason(val) => {
//            println!("Error: {}", val);
//        },
//        StringOK(val) => {
//            println!("OK: {}", val);
//        },
//    }
//}
//
//fn get_font_dir(name: &str) -> StringResult {
//    match os::consts::SYSNAME {
//        "linux" => {
//            let key = "HOME";
//            match os::getenv(key) {
//                Some(val) => StringOK(format!("{}/.fonts", val)),
//                None => ErrorReason(format!("{} is not defined in the environment.", key))
//            }
//        },
//        "macos" => {
//            StringOK("i686-apple-darwin".to_string())
//        },
//        _ => unreachable!(),
//    }
//}
//
////fn search_font(name: &str) -> &'static str {
////    let resp = http::handle()
////        .get("https://api.github.com/search/repositories?q=rfont+in:name&sort=stars&order=desc")
////        .exec.unwrap();
////    println!("body={}", resp.get_body());
////}
