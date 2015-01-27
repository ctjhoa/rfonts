#![feature(plugin)]

#[plugin]
extern crate clippy;

extern crate cli;
extern crate getopts;
extern crate hyper;

use std::os;
use std::io::fs;

fn main() {
    let opts = &[
        cli::helpopt(),
        cli::versionopt(),
        getopts::optflag("l", "list", "List installed fonts"),
        getopts::optopt("s", "search", "Search font", "FONTNAME"),
        getopts::optmulti("i", "install", "Install font(s) require a --source", "FONTNAME"),
        getopts::optmulti("d", "delete", "Delete font(s)", "FONTNAME"),
        getopts::optmulti("s", "source", "Source file to use", "FILENAME"),
    ];

    let matches = cli::parse_args(opts);
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
        search_font(&*matches.opt_str("search").unwrap());
    }
    if matches.opt_present("install") {
        if matches.opt_present("source") {
            install_font(&*matches.opt_str("source").unwrap(), &*matches.opt_str("install").unwrap());
        } else {
            println!("{}", cli::usage_string(opts));
        }
    }
    if matches.opt_present("delete") {
        delete_font(&*matches.opt_str("delete").unwrap());
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

fn get_font_path(font_name: &str) -> Path {
    get_font_dir().join(font_name)
}

fn list_installed_fonts() {
    let font_dir = get_font_dir();
    match fs::readdir(&font_dir) {
        Ok(fonts) => for font in fonts.iter() {
            if let Some(font_name) = font.filename_str() {
                println!("{}", font_name);
            }
        },
        Err(msg) => println!("{}", msg)
    }
}

fn search_font(font_name: &str) {
    let mut client = hyper::Client::new();
    let url = format!("http://api.github.com/search/repositories?q={}+in:name&sort=stars&order=desc", font_name);
    println!("{}", url);
    match client.get(&*url).send() {
        Ok(resp) => println!("body={}", resp.status),
        Err(msg) => println!("{}", msg)
    }
}

fn install_font(source: &str, font_name: &str) {
    let dest_font_path = get_font_path(font_name);
    match fs::copy(&Path::new(source), &dest_font_path) {
        Ok(_) => println!("Font {} installed successfully", font_name),
        Err(msg) => println!("{}", msg)
    }
}

fn delete_font(font_name: &str) {
    let font_path = get_font_path(font_name);
    match fs::unlink(&font_path) {
        Ok(_) => println!("Font {} deleted successfully", font_name),
        Err(msg) => println!("{}", msg)
    }
}

