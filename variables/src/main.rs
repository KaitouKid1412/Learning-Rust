fn main() {
    
    variables();

    //Difference between Shadowing and Mutating a variable
    // Shadowing allows the user to change the datatype of a variable where as mutating does not

    tuples();
    arrays();

    //Difference between Tuples and an Array
    // Tuples can have values of different types but array should have values of the same time

    //In rust we MUST declare the type of each function parameter

    expressions();

    //Difference between statements and expressions
    // Statements do not return a value whereas expressions do
    // Expressions do not have an ending semicolon whereas statements do
    // calling a function, calling a macro, new scope block created with curly braces are all expressions

    //Functions with return values :
    let v = add_one(5);
    println!("The value of v is {v}");

    //If Condition
    if_condition(10);

    //Loops
    loops();
}

fn variables() {
    //Immutable variables
    let z = 1;
    println!("The value of z is {z}");

    //Mutable Vairables
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    //Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Value of 3 hours in seconds is {THREE_HOURS_IN_SECONDS}");

    //Shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in inner scope is {y}");
    }
    println!("The value of y in outer scope is {y}");
}
fn tuples() {
    //Tuples
    let tup = (500, 6.4, 'a');
    let (_tup1, _tup2, _tup3) = tup;
    println!("The value of tup2 {_tup2}");
    // println!("The value of tup {tup}"); - This will throw an error
    // println!("The value of tup3 {tup.2}"); - This will throw an error as well
    let tuple_charater_value = tup.2;
    println!("The value of tup3 {tuple_charater_value}");
}

fn arrays() {
    //Arrays
    let _a = [1, 2, 3, 4, 5];
    let _b: [i32; 5] = [1, 2, 3, 4, 5]; //Declare an array of 5 elements of type i32
    let _c = [3; 5]; //Declare an array which has 5 elements - all of them 3s

    //Arrays are more useful when 
    // 1. When you want data allocated on stack rather than heap
    // 2. When you know the number of elements exactly and they dont change (if number of elemnts change it is better to use a vector)

    //Unlike most low-level languages, If we try to access indexes out of bounds of array (index > array length)
    // instead of returning the value in the memory, rust will throw an error and does not allow invalid memory access
}

fn expressions() {
    let y = {
        let x = 3;
        x + 1
        //Note that there is no semicolon after x+1 above - this this value will be returned to y
    };

    println!("The value of y is: {y}");
}

fn if_condition(x: u32) {
    let y = if x > 5 { 5 } else { 0 };
    println!("The value of y from if condition is {y}");
}

//Note that the '->' operator defined the return type for the function
fn add_one(x: u32) -> u32{
    x + 1
    //since we did not end the expression with the semicolon, x + 1 will be returned whenever the function is called
}

fn loops() {
    //Nester Loop with Label
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End Count = {count}");

    //While Loop
    let mut number = 3;

    while number != 0 {
        println!("number = {number}");

        number -= 1;
    }

    //Looping through a collection with 'for'
    let a = [1, 2, 3, 4, 5, 6];

    for element in a {
        println!("Current element is {element}");
    }

    //For Loop using range and rev : (1..4) gives range of numbers between 1 and 4 (both 1 and 4 included) and rev will reverse the order of the range
    for element in (1..4).rev() {
        println!("Current element in the range {element}");
    }
}