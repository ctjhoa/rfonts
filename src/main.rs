extern crate cli;
extern crate getopts;
extern crate hyper;

use std::os;
use std::io::fs::PathExtensions;
use std::io::fs;
use std::io::File;

enum StringResult {
    StringOK(String),
    ErrorReason(String),
}

fn main() {
    let opts = &[
        cli::helpopt(),
        cli::versionopt(),
        getopts::optflag("l", "list", "List installed fonts"),
        getopts::optopt("s", "search", "Search font", "FONTNAME"),
        getopts::optmulti("i", "install", "Install font(s)", "FONTNAME"),
        getopts::optmulti("d", "delete", "Delete font(s)", "FONTNAME"),
    ];

    let matches = cli::parse_args(opts);
    if matches.free.is_empty() {
        println!("{}", cli::usage_string(opts));
    }
    if matches.opt_present("help") {
        println!("{}", cli::usage_string(opts));
        return;
    }
    if matches.opt_present("version") {
        println!("{}", cli::version_string("0.0.1"));
        return;
    }
    if matches.opt_present("list") {
        list_installed_fonts();
    }
    if matches.opt_present("search") {
        //search_font(matches.opt_str("search"));
    }
}

fn get_font_dir() -> Path {
    match os::consts::SYSNAME {
        "linux" => {
            match os::homedir() {
                Some(ref p) => p.join(".fonts"),
                None => panic!("Impossible to get your home dir!")
            }
        },
        "macos" => {
            match os::homedir() {
                Some(ref p) => p.join(".fonts"),
                None => panic!("Impossible to get your home dir!")
            }
        },
        _ => unreachable!(),
    }
}

fn list_installed_fonts() {
    let font_dir = get_font_dir();
    println!("{}", font_dir.display());
    let fontPaths = fs::readdir(&font_dir).unwrap();
    for font in fontPaths.iter() {
        println!("{}", font.filename_str().unwrap());
    }
}

fn search_font(name: String) {
    let mut client = hyper::Client::new();
    let resp = client
        //.get("https://www.google.com")
        .get("https://api.github.com/search/repositories?q=rfont+in:name&sort=stars&order=desc")
        .send().unwrap();
    println!("body={}", resp.status);
}

fn install_font(name: &str) {
}

fn delete_font(name: &str) {
}

