// Use Library for inputs
use std::io;

fn main()
{
    println!("Enter Student Name: ");
    let mut student_name = String::new();
    // let mut maths_mark = String::new();
    // let mut science_mark = String::new();
    // let mut social_mark = String::new();
    let mut maths_mark = "50";
    let mut science_mark = "50";
    let mut social_mark = "50";
    let total_mark : u8;
    // let grade = String::new(); 
    io::stdin()
        .read_line(&mut student_name)
        .expect("reading input failed");
    // print!("Hi {}",student_name);
    // print!("Enter Maths Mark: ");
    // io::stdin()
    //     .read_line(&mut maths_mark)
    //     .expect("reading maths input failed");
    // print!("Enter Science Mark: ");
    // io::stdin()
    //     .read_line(&mut science_mark)
    //     .expect("reading science input failed");
    // print!("Enter Social Mark: ");
    // io::stdin()
    //     .read_line(&mut social_mark)
    //     .expect("reading social input failed");
    //Converting String to Int
    let maths_mark_u8   :u8 = maths_mark.parse().unwrap();
    let science_mark_u8 :u8 = science_mark.parse().unwrap();
    let social_mark_u8  :u8 = social_mark.parse().unwrap();
    // Finding the total
    total_mark = maths_mark_u8 + science_mark_u8 + social_mark_u8;
    print!("Total Mark is {}",total_mark)
}