
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    //Enums give us a way of saying a value is one of a possible set of values
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    //Note that variants of enum are namespaced under its identifier, they can be accessed using a double column '::'
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    //Just like how we create methods on structs, we can define methods on enums using the 'impl' keyword
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }
    let m = Message::Write(String::from("hello"));
    //The this example we have created the variable 'm' that has the value "Message::Write()" and this is what self will be
    //in the body of the call whem m.call() runs
    m.call();

    //Option Enum
    //It is defined by standard library (so we dont have to define it here again)
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    //It encodes the scenario where a value could be something or it could be nothing
    //Rust does not support Null feature, hence we have to use Option enum instead
    //Here <T> syntax is a generic type parameter (which we will talk more about later)
    //<T> means that 'Some' variant of 'Option' enum can hold one piece of data of any type.
    //And that each concrete type which gets used instead of <T> will make 'Option' a different type
    //This means that Option<i32> and Option<String> are two different types
    //Option enum is included in the preluds so we do not have to bring it into scope explicity, we can even use Some() and None without using the 'Option::' pretext
    let _some_number = Some(5);
    let _some_char = Some('e');
    //When we are using 'Some', we do not have to explicity mention the type since Rust compiler can infer it based on the 'T' we are passing
    let _absent_number: Option<i32> = None;
    //However, when we are using 'None' we have to mention the type explicitly.
    //Note that Option<i32> and i32 are different types and we cannot do operations like Option<i32> + i32.

    //Match Control Flow Construct
    //Match allows us to compare a value against a series of patterns and execute code based on which pattern matches
    
    //If vs Match
    //If condition needs to evaluate to boolean whereas match can be any type
    let penny = Coin::Penny;
    value_in_cents(penny);

    //Example : Matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);
    let none = plus_one(None);
    println!("{:?}", none);

    //Matches are Exhaustive! Match's arms must cover all patterns otherwise the code will not combine

    //Catchall patterns and the _ Placeholder
    //We can use 'other' pattern in match so that every pattern which is possible but not mentioned can be handled by this scenario
    //We can also use '_' pattern to match all possibilities which are not mentioned in the match pattern
    //Difference between 'other' and '-' is that 'other' will use the value of the variable where as '_' will not
    //Note that 'other' and '_' should only be used at the end of the match pattern since if we use them at the beginning of the match, all other cases will not execute

    //If Let Control Flow
    //Instead of using one pattern for match case and using catchall patterns, we can use 'if let' control flow
    //For Example, instead of the following match
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    //We can use the following 'if let' control flow to make it less verbose
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
