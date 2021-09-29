use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("iCoin Blockchain Platform");
    let width = message.chars().count();


    let mut Writer = BufWriter::new(stdout.lock());

    say(message.as_bytes(), width, &mut Writer).unwrap();
}
