fn main() {
    //Ownership features in Rust allows it to make memory safety guarantees without needing a garbage collector
    //All programs have to manage computer's memory while running
    //Some languages have garbage collection that regularly looks for no longer used memory
    //In others, the programmer must explicity allocate and fee the memory
    //Rust uses a third approach : Memory is managed through a system of ownership with a set of rules that compiler checks.

    //Stack vs Heap
    stack_vs_heap();

    //Ownership Rules
    ownership_rules();

    //Scope
    scope();

    //Memory and Allocation
    memory_and_allocation();

    //Move vs Close
    move_vs_clone();

    //Ownership and Functions
    ownership_and_functions();

    //Return Values and Scope
    return_values_and_scope();

    //What if we want to pass a variable to a function, but dont want the function to take ownership of the variable ? 
    //We can pass a variable to a function without transfering its ownership using Referrencing
    // To Referance a variable, use '&' 
    // To De-referance a variable, use '*'
    referencing_and_borrwing();

    //What if we want to modify something we have a reference to ? 
    mutable_references();

    //Dangling References
    //Dangling pointer is a pointer that references a memory which has been freed. and the pointer was not handled correctly so it still points to the freed memory
    //Rust automatically handles dangling pointers and throws error at compile time

    //Slices
    slices();
}

fn stack_vs_heap() {
    //Stack is LIFO, all data stored on stack must have a fixed size, data with unknown size at compile time must be stored in heap instead.
    //When you request memory on heap, memory allocator finds an empty spot on the heap and returns a pointer. This pointer is stored in the stack, whenever we want to access this data, we follow the pointer.
    //Pushing data to stack is faster than heap because we always allocate memory at the top of the stack whereas in heap, we have to find a place in data which has the required amount of size, create pointer, etc.
    //Similarly, accessing data from stack is faster than heap because we need to use pointers to access heap memory
    //When you call a function, all the functions local variables and arguments get pushed onto the stack, and once the function is over these values get popped off
}

fn ownership_rules() {
    //Main purpose of ownership is to manage heap data
    //Keeping track of what parts of code are using what data on the heap,
    //minimizing the amount of duplicate data on the heap,
    //cleaning up unused data on the heap so the program does not run out of memroy
    //are all problems that ownership addresses.

    //Ownership Rules :
    // - Each value in Rust has an owner
    // - There can only be one owner at a time
    // - If the owner goes out of scope, the value will be dropped
}

fn memory_and_allocation() {
    //When the amount of memory to be allocated to a value is not known during compile time, 
    //we have to use an 'allocator' to allocate memory during run time.

    //1. The memory has to be allocated by the 'allocator' during run time
    //2. The memory has to be freed up once it has been used

    //First step is pretty straightforward,
    //Whereas, the second step is handled by languages in two ways -
    // a. Run a Garbage Collector
    // b. The programmer has to handle allocation and freeing up of memory

    //Rust usees a THIRD approach where the memory allocation is handled by a process of ownership
    //Memory is owned by components and freed as soon as the component goes out of scope
}

fn scope() {
    //Scope is a range within a program for which an item is valid
    {
        //s is not valid here since it is not yet declared
    let _s = "hello"; //s is valid from this point onward

    //perform actions with or on s
    } 
    // this scope is over now, s is no longer valid, the memory will be freed and returned to allocator.
    //To free this memory, rust calls a special function called 'drop'
}

fn move_vs_clone() {
    //Case 1
    let x = 5;
    let _y = x;
    //The above code binds the value 5 to X and makes a copy of value in X and bings it to Y

    //Case 2
    let s1 = String::from("hello");
    let _s2 = s1;
    //In the above case, the metadata for variable for s1 is stored in the stack, whereas the actual value (hello) is stored on the heap.
    //The pointer for the value of s1 is stored as a part of metadata on the stack,
    //When we store s2 = s1, we create a copy of s1's metadata and store it on the stack, and now s2 points to the same reference on the stack that s1 was previously pointing
    //In this case, once we assign s1 to s2, Rust calls drop on s1 and the value is removed from stack.
    //This is to prevent 'double fee' error - where if both s1 and s2 go out of scope, the program tries to free the same memory twice causing error
    //In the above case, the s2 = s2 uses the move function in rust which moves the pointer from s1 to s2 and frees up s1 memory on stack

    //If we want to keep both s1 and s2, we can use the 'copy' function in rust
    let s3 = String::from("hello");
    let _s4 = s3.clone();
    //In the above case, Rust copies both the metada of s1 on stack, as well as the value on heap and points s3 to the new memory copied on Heap.
    //For more information, refer - https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move

    //We spoke about two scenarios above, how will we know how our variable will be handled ? 
    //Rust has 'traits' which types can implement.
    //There is a special trait called 'Copy' which, if implemented by a type, will act as 'Case 1' mentioned above.
    //Note that any type which has 'Drop' type (discussed in the previous function) cannot have 'Copy' type
}

fn ownership_and_functions() {
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn return_values_and_scope() {
    let _s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1
    let s2 = String::from("hello");     // s2 comes into scope
    let _s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope
    a_string  // a_string is returned and moves out to the calling function
}

fn referencing_and_borrwing() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { //When declarig the parameters, we mention '&' to specify that s is a referance variable
    s.len() // Note that we cannot change the value of s in this function
} // The value of s is dropped after the scope, but the variable which is referrenced to (s1 in this case) is not dropped after this scope

fn mutable_references() {
    //If we want to modify a referrence (borrowed value) we will have to pass it as a reference
    let mut s = String::from("hello"); //we have to declare s as a mutable variable
    change(&mut s);

    //Mutable references have one huge restriction.
    //If we can only have one mutable referance to a value
    //This restriction is placed to prevent 'data race'
    //We cannot combine mutable and immutable references either
    //However, multiple immutable references are allowed

    //However we can use curly braces to create a new scope, allowing for multiple mutable referances, just not simultaneous ones
    let mut s = String::from("hello");
    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let _r2 = &mut s;

    //Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. 
    //For instance, this code will compile because the last usage of the immutable references, the println!, occurs before the mutable reference is introduced:
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);

}

fn change(some_string: &mut String) { //We declare the parameters as mutable references
    some_string.push_str(", world");
}

fn slices() {
    //In the context of this notes, we will talk about String slices for ease of understanding
    //String slices are a reference to a part of a string

    let s = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];
    //The above code can also be modified as - 
    let _hello = &s[..5];
    let _world = &s[6..];
}
