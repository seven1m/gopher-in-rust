extern crate clap;
extern crate url;
extern crate regex;

use std::str;
use std::io::prelude::*;
use std::net::TcpStream;
use clap::{App, Arg};
use url::Url;
use regex::Regex;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("gopher-in-rust")
        .version(VERSION)
        .about("gopher client")
        .arg(Arg::with_name("URL")
             .required(true)
             .index(1))
        .get_matches();

    let url_string = matches.value_of("URL").unwrap().replace(" ", "%20");
    let url = match Url::parse(&url_string) {
        Ok(url) => url,
        Err(_) => Url::parse(&format!("gopher://{}", url_string)).unwrap()
    };
    let host = format!("{}:{}", url.host_str().unwrap(), url.port_or_known_default().unwrap());
    let mut stream = TcpStream::connect(host).unwrap();
    let path = url.path().replace("%20", " ");
    if path != "/" {
        stream.write_all(path.as_bytes()).unwrap();
    }
    stream.write_all(b"\r\n").unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    let menu_re = Regex::new(r"^[i0123456789I].+\t.+\t.+\t.+").unwrap();
    if menu_re.is_match(&response) {
        for line in response.lines() {
            if line == "." {
                break;
            } else if line.len() > 0 {
                let (line_type, line) = line.split_at(1);
                let parts: Vec<String> = line.split("\t").map(String::from).collect();
                match line_type {
                    "i" => { println!("{}", parts[0]) },
                    "0" => { println!("link: {} ({})", parts[0], parts[1]) }
                    "7" => { println!("search: {} ({})", parts[0], parts[1]) }
                    _ => {}
                }
            } else {
                println!();
            }
        }
    } else {
        println!("{}", response);
    }
}
