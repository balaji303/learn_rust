fn main()
{
    // Unsigned int8_t
    let temp_var:u8 = 255;
    println!("{}",temp_var);

    // Signed int8_t
    let temp_var:i8 = -125;
    println!("{}",temp_var);

    //string to integer
    let string_to_int: u32 = "100".parse().expect("string_to_int is not a number!");
    println!("{}",string_to_int);

    //Hexadecimal
    let hexa: i64 = 0x0000000f;
    println!("{}",hexa);

    //Octal
    let oct = 0o7;
    println!("{}",oct);

    // Binary
    let bin = 0b0100;
    println!("{}",bin);

    //Character
    let byte = b'A';
    println!("{}",byte);
    
    // Printing emojis
    let byte = '🚀';
    println!("{}",byte);

    //Float values are f64 by default
    let temperature = 26.45;
    println!("{}",temperature);

    //Float values also support f32
    let temperature:f32 = 26.44;
    println!("{}",temperature);

}