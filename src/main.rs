//#![feature(plugin)]
//
//#[plugin]
//extern crate clippy;

extern crate cli;
extern crate getopts;
extern crate hyper;

use std::os;
use std::env;
use std::old_io::fs;
use getopts::Options;

static FONT_EXTENSIONS : [&'static str; 2] = ["ttf", "otf", "pcf", "bdf"];

fn main() {
    let mut opts = Options::new();
    cli::helpopt(&mut opts);
    cli::versionopt(&mut opts);
    opts.optflag("l", "list", "List installed fonts");
    opts.optopt("s", "search", "Search font", "FONTNAME");
    opts.optmulti("i", "install", "Install font(s) require a --source", "FONTNAME");
    opts.optmulti("d", "delete", "Delete font(s)", "FONTNAME");
    opts.optmulti("s", "source", "Source file to use", "FILENAME");

    let matches = cli::parse_args(&opts);

    if matches.opt_present("help") {
        println!("{}", cli::usage_string(&opts));
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
            println!("{}", opts.usage("ttoo"));
        }
    }
    if matches.opt_present("delete") {
        delete_font(&*matches.opt_str("delete").unwrap());
    }
}

fn get_font_dir() -> Path {
    match env::consts::OS {
        "linux" => {
            match env::home_dir() {
                Some(ref p) => p.join(".fonts"),
                None => panic!("Impossible to get your home dir!")
            }
        },
        "macos" => {
            match env::home_dir() {
                Some(ref p) => p.join("Library").join("Fonts"),
                None => panic!("Impossible to get your home dir!")
            }
        },
        "windows" => {
            match os::getenv("SystemRoot") {
                Some(val) => Path::new(val).join("Fonts"),
                None => panic!("Impossible to get your font dir!")
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
        Ok(fonts) => {
            let mut it_fonts = fonts.iter().filter(|&f| {
                FONT_EXTENSIONS.iter().find(|&ext| {
                    *f.extension_str().unwrap().to_string() == *ext.to_string()
                }).is_some()
            });
            for font in it_fonts {
                if let Some(font_name) = font.filename_str() {
                    println!("{}", font_name);
                }
            }
        }
        Err(msg) => println!("{}", msg)
    }
}

fn search_font(font_name: &str) {
    let mut client = hyper::Client::new();
    let url = format!("http://api.github.com/search/repositories?q={}+in:name&sort=stars&order=desc", font_name);
    let resp = client.get(&*url).send();
    match resp {
        Ok(mut data) => match data.read_to_string() {
            Ok(body) => println!("body={}", body),
            Err(err) => println!("{}", err)
        },
        Err(err) => println!("{}", err)
    };
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

