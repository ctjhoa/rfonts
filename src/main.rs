//#![feature(plugin)]
//
//#[plugin]
//extern crate clippy;

//extern crate cli;
#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate hyper;
extern crate docopt;
extern crate rustc_serialize;

use std::{env, fs, path};
use std::io::Read;
use std::process::Command;

use docopt::Docopt;

static FONT_EXTENSIONS : [&'static str; 4] = ["ttf", "otf", "pcf", "bdf"];
static WIN_FONT_REGISTRY : &'static str = "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Fonts";

docopt!(Args derive Debug, "
Rusty fonts. Simple font manager written in rust made for 

Usage:
    rfonts list
    rfonts search <font-name>
    rfonts install <source> <font-name>
    rfonts delete <font-name>
    rfonts (-h | --help)
    rfonts --version

Options:
    -h --help     Show this screen.
    --version     Show version.
");

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("rfonts version 0.0.1");
        return;
    }
    if args.cmd_list {
        list_installed_fonts();
    }
    if args.cmd_search {
        search_font(&*args.arg_font_name);
    }
    if args.cmd_install {
        install_font(&*args.arg_source, &*args.arg_font_name);
    }
    if args.cmd_delete {
        delete_font(&*args.arg_font_name);
    }
}

fn get_font_dir() -> path::PathBuf {
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
            match env::var_os("SystemRoot") {
                Some(ref val) => path::Path::new(val).join("Fonts"),
                None => panic!("Impossible to get your font dir!")
            }
        },
        _ => unreachable!(),
    }
}

fn get_font_path(font_name: &str) -> path::PathBuf {
    get_font_dir().join(font_name)
}

fn list_installed_fonts() {
    let font_dir = get_font_dir();
    match fs::read_dir(&font_dir) {
        Ok(entries) => {
            let files : Vec<_> = entries.filter_map(|f| {
                f.as_ref().ok().and_then(|dir_entry| {
                    Some(dir_entry.path())
                })
            }).collect();
            
            for path in files {
                if let Some(ext) = path.extension() {
                    if FONT_EXTENSIONS.contains(&&*ext.to_string_lossy()) {
                        if let Some(file_name) = path.file_name() {
                            println!("{}", file_name.to_string_lossy());
                        }
                    }
                }
            }
        },
        Err(msg) => println!("{}", msg)
    }
}

fn search_font(font_name: &str) {
    let mut client = hyper::Client::new();
    let url = format!("http://api.github.com/search/repositories?q={}+in:name&sort=stars&order=desc", font_name);
    let resp = client.get(&*url).send();
    match resp {
        Ok(mut data) => {
            let mut body = String::new();
            data.read_to_string(&mut body).ok();
            println!("body={}", body);
        },
        Err(err) => println!("{}", err)
    };
}

fn install_font(source: &str, font_name: &str) {
    let dest_font_path = get_font_path(font_name);
    match fs::copy(&path::Path::new(source), &dest_font_path) {
        Ok(_) => println!("Font {} installed successfully", font_name),
        Err(msg) => println!("{}", msg)
    };
    match env::consts::OS {
        "windows" => {
            let output = Command::new("reg").arg("add").arg(WIN_FONT_REGISTRY)
                .arg("/v").arg(format!("{} (TrueType)", font_name))
                .arg("/t").arg("REG_SZ")
                .arg("/d").arg(format!("{}", font_name))
                .arg("/f")
                .output().unwrap_or_else(|e| {
                    panic!("failed to execute process: {}", e)
                });
            if output.status.success() {
                let s = String::from_utf8_lossy(&output.stdout);
                println!("Post-install scripts:");
                print!("{}", s);
            }
        },
        _ => {},
    };
}

fn delete_font(font_name: &str) {
    let font_path = get_font_path(font_name);
    match fs::remove_file(&font_path) {
        Ok(_) => println!("Font {} deleted successfully", font_name),
        Err(msg) => println!("{}", msg)
    }
}

