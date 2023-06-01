fn main()
{
    let account_balance = 5000;
    println!("Your Account Balance is: {}", account_balance);
    //The below method throughs error because variables are immutable(unchangeable) by default 
    // account_balance = 50;
    // println!("Your Account Balance is: {}", account_balance);
    let account_balance = 50;
    println!("Your Account Balance is: {}", account_balance);

    // But if mutable variable are created they can be modified
    let mut loan_amount = 5000;
    println!("Your Loan Amount is: {}", loan_amount);

    // This is a mutable variable so it can be modified
    loan_amount = 8000;
    println!("Your Loan Amount is: {}", loan_amount);

    let account_name = "Balaji";
    println!("Your name is: {}", account_name);

    //Const Values
    const MAX_WITHDRAW_LIMIT: u32 = 5000;
    println!("Max withdraw limit is: {}", MAX_WITHDRAW_LIMIT);

    //Shadowing
    let spaces = "    "; //4 Spaces
    let space_len = spaces.len();
    println!("Length of spaces is: {}", space_len);
}