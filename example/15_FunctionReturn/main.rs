fn function_return(value: u8) -> bool
{
    if value > 100
    {
        return true;
    }
    else
    {
        return false;
    }
} 

fn main()
{
    let mut bool_value = function_return(5);
    println!("Value is {}",bool_value);
    bool_value = function_return(200);
    println!("Value is {}",bool_value);
}