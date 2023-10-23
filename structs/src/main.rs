//Defining a struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//Method : Unlike functions, methods are defined within the context of a struct, or enum or trait object.
//The first parameter of Method is always self, which represents the instance of the struct, enum or trait being called on
impl Rectangle {
    //Here we pass &self which is a short hand for self: &Self
    //here we are passing an immutable reference of Rectangle (Self) object to this function
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    //Note that we can give method name same at that of struct's fields
    //Often, but not always, when we give a method the same name as a field we want it to only return the value in the field and do nothing else.
    //Methods like this are called getters, and Rust does not implement them automatically for struct fields as some other languages do.
    //Getters are useful because you can make the field private but the method public, and thus enable read-only access to that field as part of the type’s public API.
    fn width(&self) -> bool {
        self.width > 0
    }

    //We can give multiple parameters after the self parameter to a method
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //Associated Functions are functions that do not need self as the first parameter (and hence are not methods)
    //because they dont need instance of the type (struct in this case) to work with
    //Associated functions are often used for contructors which return a instance of the struct 
    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }
}

fn main() {
    //To use a struct after we have defined it, we create an instance of the struct by specifying concrete values for each fields
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    //We can access the values from a struct using dot notaion
    println!("{}", user1.active);
    //However, we cannot change the values using dot notation unless the entire struct is mutable
    //We cannot change the above struct however, we can change the below struct
    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    //Since the above struct is mut, so we can change the values using dot notation
    user2.email = String::from("anotheremail@example.com");
    //Note that the entire instance has to be mutable - we cannot have a few mutable fields

    //Field Init Shorthand
    build_user(String::from("someusername123"), String::from("someone@example.com"));

    //Struct Update Syntax
    //Every other value will be kept the same as user1 except email which will be changed
    let _user3 = User {
        email: String::from("another@example.com"),
        //The following code should come at last to specify remaining elements are same as user1
        ..user1
    };
    //IMPORTANT NOTE - in the above code, we can no longer user user1 because username field of user1 has been 'MOVED' (check ownership) to user3
    //However, if we gave a new value of email and username for user3 and used update syntax only for active and sign_in_count,
    //user1 would still be valid since both active and sign_in_count types apply 'COPY' trait (which is not applied by email and username)

    //Tupled Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    //Each struct we define is a different type although they are made up of same type (i32 in the above case)
    //A function which takes Color as input cannot be passed Point, it will throw an error
    //To access an individual value in tupled structs, you use a '.' followed by index

    //Unit-like structs
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
    //Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself

    //Method Syntax
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    //To call Associated Functions, we use '::' suyntax along with the struct name
    let _sq1 = Rectangle::square(30);
}

fn build_user(email: String, username: String) -> User{
    User{
        active: true,
        //we can use the following shorthand instead of username: username
        username,
        email,
        sign_in_count: 1,
    }
}
