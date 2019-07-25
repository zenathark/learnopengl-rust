use std::env;

mod chap01;

fn main() {
    if env::args().count() < 3 {
        eprintln!("Not enough argumenst. Usage: chap subchap exercise.");
        std::process::exit(1);
    }
    let chap: Vec<i32> = env::args().skip(1).map(|e: String| e.parse::<i32>().unwrap()).collect();
    match chap[..3] {
        [1, 1, 1] => { chap01::main0101(); }
        [1, 1, 2] => { chap01::main0102(); }
        _ => { println!("Unknown option"); }
    }
}
