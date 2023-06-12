fn main()
{
    let value = 6;
    match value
    {
        1=>println!("one"),
        2=>println!("Two"),
        // More than one number check
        3|4=>println!("Three or Four"),
        // Range Check
        5..=7=>println!("Between 5 and 7"),
        // 10..=>println!("Greater than 10"), // Not valid because end is not specified
        _=>println!("Something"),
    }
}