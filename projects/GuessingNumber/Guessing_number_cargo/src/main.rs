use rand::Rng;
use std::cmp::Ordering::*;

fn main() {
	let mut rnd_num = rand::thread_rng();
    let computer_num: u8 = rnd_num.gen();
	println!("Random number : {}", computer_num);
    let human_num = 55;
    match computer_num.cmp(&human_num)
    {
        Less => println!("Less"),
        Greater => println!("Greater"),
        Equal => println!("Equal")
    }
}
