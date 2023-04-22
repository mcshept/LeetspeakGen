use std::io;
use std::io::*;

fn main() {

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
