fn main()
{
    let value = "two";
    match value
    {
        "one"=>println!("one"),
        "two"=>println!("Two"),
        // More than one number check
        "three"|"four"=>println!("Three or Four"),
        _=>println!("Something"),
    }
}