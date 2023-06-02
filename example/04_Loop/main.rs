
fn main()
{
    let mut loop_count = 0;

    loop
    {
        println!("loop_count is: {}", loop_count);
        if loop_count == 2
        {
            break;
        }
        loop_count += 1;
    };
}