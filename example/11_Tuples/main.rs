fn main()
{
    // Tuples
    let tuples_var = (10, 20, 30.6);
    // Tuples has circle bracket
    let (a, b, c) = tuples_var;
    println!("a:{}, b:{}, c:{}",a,b,c);
    // Tuple Indexing
    println!("Index 1:{}\nIndex 2:{}\nIndex 3:{}",tuples_var.0, tuples_var.1, tuples_var.2);
}