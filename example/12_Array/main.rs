fn main()
{
    let names = ["aaa", "bbb", "ccc", "ddd"];
    println!("Name: {}",names[0]);
    let name_b = names[1];
    println!("Names b: {}",name_b);
    println!("Names Joined {}",names.join("-"));

    //          [Types: size]
    let var_64: [i64;5] = [1, 2, 3, 4, 5];
    println!("var 64: {}",var_64[2]);

    // [Number; Number of times] // ! its semicolon in middle not comma
    let lazy_method = [1;3]; // [1,1,1]
    println!("lazy_method 0:{}\n lazy_method 1:{}\n lazy_method 2:{}",lazy_method[0], lazy_method[1], lazy_method[2]);

}