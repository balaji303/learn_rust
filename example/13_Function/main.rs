fn main()
{
    hello();
    hello();
    add(32,555);
}

fn hello()
{
    //Its only print
    print!("Hello");
}

fn add(a: i32, b:i32)
{
    let c = a + b;
    println!("sum of value: {}",c);
}