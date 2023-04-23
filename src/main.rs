use std::io;
use std::io::*;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::str::from_utf8;
use reqwest::blocking::{Client, get};
use std::io::{ErrorKind, copy, Seek, SeekFrom};
use reqwest::Url;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::PathBuf;
use std::result::Result;

use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking;
use std::error::Error;

use std::process::{Command, Stdio};

fn download_file_with_progress(url: &str, output_file: &PathBuf) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).send()?;
    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;
    let mut output = File::create(output_file)?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"),
    );

    let mut content = Cursor::new(response.bytes()?);
    let mut buffer = [0; 8_192];

    loop {
        let len = content.read(&mut buffer)?;
        if len == 0 {
            break;
        }
        downloaded += output.write(&buffer[0..len])? as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("[✔️] Download complete");

    Ok(())
}


fn get_file_content(url: &str) -> String {
    let mut response = reqwest::blocking::get(url).unwrap();
    let mut content = String::new();
    response.read_to_string(&mut content).unwrap();
    content
}


fn main() {
    let version = "1.0.1-hotfix-2";

    println!("[⌛] Checking...");
    let url = "https://raw.githubusercontent.com/mcshept/LeetspeakGen/master/ver.txt";
    let content = get_file_content(url);
    if content.trim().eq_ignore_ascii_case("404: Not Found") {
        println!("[❌] Error: Could not find version file!");
        println!("Press any key to exit...");
        let mut exit = String::new();
        io::stdin().read_line(&mut exit).expect("error: 1x11111");
    } else {
        println!("[✔️] Done!\nLatest Version: {}    Your Version: {}", content, version);
        if content.trim().eq(version) {
            println!("[✔️] You are using the latest version!");
        } else {
            println!("[⚠️] You are not using the latest version!");
            println!("Do you want to update?\n 1. (Y)es\n 2. (N)o");
            let mut update = String::new();
            io::stdin().read_line(&mut update).expect("error: 1x11111");
            match update.trim().to_lowercase().as_str() {
                "y" | "yes" => {
                    println!("[⌛] Downloading new file...");
                    let url = "https://github.com/mcshept/LeetspeakGen/releases/latest/download/LeetspeakGen.Installer.exe";
                    let output_file = PathBuf::from("LeetspeakGen.Installer.exe");

                    match download_file_with_progress(url, &output_file) {
                        Ok(()) => {
                            println!("[✔️] Download complete");
                            Command::new(output_file)
                                .spawn()
                                .expect("Failed to execute command");

                            std::process::exit(0);
                        }
                        Err(err) => {
                            println!("[❌] Error: {}", err);
                            println!("Press any key to exit...");
                            let mut exit = String::new();
                            io::stdin().read_line(&mut exit).expect("error: 1x11111");
                            std::process::exit(0);
                        }
                    };
                }
                "n" | "no" => {
                    println!("[✔️] You can update later by running the program next time!");
                }
                _ => {}
            }
        }

        println!(" ");
        println!("Welcome how do you wanna use the generator?\n\n1. Convert a normal text to Leetspeak\n2. Convert Leetspeak to normal text");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("error: 1x11111");
        match choice.trim() {
            "1" => {
                println!("Enter a text to convert:");
                let mut n_text = String::new();
                io::stdin().read_line(&mut n_text).expect("error: 1x11111");
                println!("{}", n_text.to_lowercase()
                    .replace("a", "4")
                    .replace("b", "8")
                    .replace("c", "c")
                    .replace("d", "d")
                    .replace("e", "3")
                    .replace("f", "f")
                    .replace("g", "9")
                    .replace("h", "h")
                    .replace("i", "i")
                    .replace("j", "j")
                    .replace("k", "k")
                    .replace("l", "1")
                    .replace("m", "m")
                    .replace("n", "n")
                    .replace("o", "0")
                    .replace("p", "p")
                    .replace("q", "q")
                    .replace("r", "r")
                    .replace("s", "5")
                    .replace("t", "7")
                    .replace("u", "u")
                    .replace("v", "v")
                    .replace("w", "w")
                    .replace("x", "x")
                    .replace("y", "y")
                    .replace("z", "2"));
                println!("Press any key to exit...");
                let mut exit = String::new();
                io::stdin().read_line(&mut exit).expect("error: 1x11111");
            }
            "2" => {
                println!("Enter a text to convert:");
                let mut l_text = String::new();
                io::stdin().read_line(&mut l_text).expect("error: 1x11111");
                println!("{}", l_text.to_lowercase()
                    .replace("4", "a")
                    .replace("8", "b")
                    .replace("c", "c")
                    .replace("d", "d")
                    .replace("3", "e")
                    .replace("f", "f")
                    .replace("9", "g")
                    .replace("h", "h")
                    .replace("i", "i")
                    .replace("j", "j")
                    .replace("k", "k")
                    .replace("1", "l")
                    .replace("m", "m")
                    .replace("n", "n")
                    .replace("0", "o")
                    .replace("p", "p")
                    .replace("q", "q")
                    .replace("r", "r")
                    .replace("5", "s")
                    .replace("7", "t")
                    .replace("u", "u")
                    .replace("v", "v")
                    .replace("w", "w")
                    .replace("x", "x")
                    .replace("y", "y")
                    .replace("2", "z"));
                println!("Press any key to exit...");
                let mut exit = String::new();
                io::stdin().read_line(&mut exit).expect("error: 1x11111");
            }
            _ => {
                println!("Please select option 1 or 2!")
            }
        }
    }
}
