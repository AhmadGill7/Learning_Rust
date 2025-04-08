// Memory Management in RUST

/// STACK and HEAP memories
///
/// in stack memory the int and bools are stored and in heap memory the Strings and Vectors are stored
/// why? because mostly the int and bools bytes are static they dont change and the Strings and Vectors are dynamic they change
/// like if there is a string "hello" is initially stored in the heap memory then if we append " world" to it then the old string
/// will be modified and its position can be changed or can be the same depending on the memory available in the RAM from the
/// for strings : the length capacity and pointer are stored in the stack memory , the pointer point to the location in the heap where
/// the actual string is stored.
/// if we modify the string it will check if the memory is available in the heap memory then it will be added there or it will be moved from there if
/// there is no memory available in the heap memory right ext to its previous location in the heap memory.
///
///  

// fn main() {
//     stack_fn();
//     heap_fn();
//     update_str_fn()
// }

// fn stack_fn() {
//     let x = 5;
//     let y = 10;
//     println!("The value of x is {} {1}", x, y);
// }

// fn heap_fn() {
//     let s1 = String::from("Hello");
//     let s2: String = String::from("World");
//     let combined = format!("{} {}", s1, s2);
//     println!("{}", combined);
// }

// fn update_str_fn() {
//     let mut s1 = String::from("Hello");
//     println!("Before Update this was the string `{}`", s1);
//     println!(
//         "and this was its capacity {}, length {}, and pointer {:p} before the update",
//         s1.capacity(),
//         s1.len(),
//         s1.as_ptr()
//     );

//     for i in 0..10 {
//         s1.push_str("A Random Line");
//         println!(
//             "In the Loop this was its capacity {}, length {}, and pointer {:p} at index {}",
//             s1.capacity(),
//             s1.len(),
//             s1.as_ptr(),
//             i
//         );
//     }
// }

// ******** OwnerShip ********

/// OwnerShip
///
/// 1. Each value has a variable that's called its owner.
/// 2. There can only be one owner at a time.
/// 3. When the owner goes out of scope, the value will be dropped.
///
/// we can transfer the ownership of a value to another variable by moving it or we can copy it so the same value can be used by multiple variables
/// but we can say that it has two owners at the same time
/// when the owner goes out of scope then the value will be dropped and the value will be deleted
/// that's why rust is a memory safe language

// fn main() {
//     let s: String = String::from("Hello");
//     let s1 = s; // as the s is moved to s1, s is no longer valid
//     // println!("The value of x is {}", s); // so we cant access it
//     println!("The value of s is {}", s1); // but we can access x1 now

//     let string_one = String::from("Hello");
//     println!("before moving the ownership {}", string_one); // we can access the string_one here because it is the owner till now but when the ownership moves to the function ots gone BECAUSE the function is not returning anything.
//     // uncomment this part
//     // take_ownership(string_one);
//     // println!("{}", string_one); // now the ownership is moved to the take_ownership function thats why we can access it here
//     // error here
//     // Can we Fix This? yes we can....by returning it from the function or by cloning it thats one option but its an expensive option for the heap and the second opt is by borrowing it

//     let string_two = take_ownership_with_return(string_one);
//     println!("{}", string_two); // now the ownership is moved to the take_ownership function thats why we can access it here
//     // Can we Fix This? yes we can....by returning it from the function or by cloning it thats one option but its an expensive option for the heap and the second opt is by borrowing it

//     // how ownership works For integers and booleans
//     let x = 5;
//     let x1 = x; // now we have copied the value of x to x1 but because it is only in stack we can assess x variable also
//     println!("The value of x is {} ant this is the value of x1 {}", x, x1); // here is the example of ownership for integers

//     //  for Booleans
//     let b = true;
//     let b1 = b; // its same for booleans like it is for integers
//     println!("The value of b is {} ant this is the value of b1 {}", b, b1); // because its in the stack

//     // ownership works for Strings and vectors because these are stored on the heap and are changeable
//     //for Bools and Integers the ownership don't work for them as it works for Strings and Vectors or anything in the heap memory
// }

// unComment this too for checking the ownership transfer
// fn take_ownership(some_string: String) {
//     println!("After moving the ownership {}", some_string);
// }
#[allow(dead_code)]
fn take_ownership_with_return(some_string: String) -> String {
    println!("After moving the ownership {}", some_string);
    return some_string;
}
// ******** Borrowing ********

// fn main() {
//     let mut string_one = String::from("Hello");
//     println!("string_one is this `{}`", string_one);
//     let string_two = &string_one;
//     println!(
//         "As string_two is borrowing the reference of string_one it is the same as in string_one `{}`",
//         string_two
//     );

//     // we can also borrow the values to functions even if it returns or not and we'll still have the original owner of the value

//     borrow_string(&string_one);
//     println!(
//         "string_one is the same as you can see even after passing it to the function `{}`",
//         string_one
//     );
//     // we can mutate the value by passing it to the function like mentioned above but we can say that you can borrow a mutable reference
//     borrow_and_mutate(&mut string_one); // it did not returned any value but ut has changed/mutated the value of string_one
//     println!(
//         "string_one is the same as you can see even after passing it to the function `{}`",
//         string_one
//     );

//     borrow_string(&string_one);
//     println!(
//         "now if we again borrow this it will be the same as it was after the mutating function call `{}`",
//         string_one
//     );
// }

// fn borrow_string(some_string: &String) {
//     println!("The borrowed string in the function is `{}`", some_string);
// }

// fn borrow_and_mutate(some_string: &mut String) {
//     some_string.push_str(" World");
//     println!(
//         "The borrowed and mutated string in the function is `{}`",
//         some_string
//     );
// }

// ********************************************************
// ********************************************************
// ********************************************************
// ********************************************************
// ********************************************************
// ********************************************************
// ********************************************************

/// **************** Some Rules For Borrowing ****************
///
/// you can borrow a variable as many times as you want but it cant be a mutable borrowing reference
/// if you have a mutable reference then you can't have a immutable reference which means you
///  can only use one mutable reference at a time and no other borrowing is allowed even if its a mut reference or a simple reference
///
/// case 1
/// value is loyal to the owner , there is no borrowing
///
/// case 2
/// value is borrowed by other but it is not mutable by any other borrowing (&mut) reference in this case
/// you can borrow it as many times as you want
///
/// case 3
/// if you want to borrow a mutable reference of a var then you can only borrow it once at a time no matter other are simple borrowings or mutable borrowings

// fn main() {
//     let mut s1 = String::from("Hello");
//     // let s2 = &mut s1; // commenting because i cant borrow that in the next line of code
//     update_word(&mut s1); // you cant borrow two mutable references of a single variable at the same time
//     println!("{}", s1);
//     // println!("{}", s2);
// }

// fn update_word(word: &mut String) {
//     word.push_str(" World");
// }

// ************************************
// Practice

// Error 1

// Fix the ownership issues in this code
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("{}, world!", s1);
// }

// Fix:-

// fn main() {
//     let s1 = String::from("hello");
//     let _s2 = &s1;
//     println!("{}, world!", s1);
// }

// error 2
// fn main() {
//     let s = String::from("ownership");
//     print_string(s);
//     println!("Can I still use s here? {}", s); // Fix this
// }

// fn print_string(s: String) {
//     println!("{}", s);
// }

// fix :-

// fn main() {
//     let s = String::from("ownership");
//     print_string(&s);
//     println!("Can I still use s here? {}", s); // Fix this
// }

// fn print_string(s: &String) {
//     println!("{}", s);
// }

// error 3

// fn main() {
//     let mut x = 5;
//     increment(&mut x);
//     println!("x is now {}", x);
// }

// // Implement this function to increment x by 1
// fn increment(x: &mut i32) {
//     // Your code here
// }

// fix :-

// fn main() {
//     let mut x = 5;
//     increment(&mut x);
//     println!("x is now {}", x);
// }

// // Implement this function to increment x by 1
// fn increment(x: &mut i32) {
//     // Your code here
//     *x += 1; // * is the dereference operator it is used to access the value of the variable
// }

// error 4 Create a Rectangle Struct

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     // 1. Create a rectangle instance
//     // 2. Print its width and height
//     // 3. Write an area function that takes a Rectangle and returns area
// }

// fix :-

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     // 1. Create a rectangle instance
//     // 2. Print its width and height
//     // 3. Write an area function that takes a Rectangle and returns area
//     let rect = Rectangle {
//         width: 10,
//         height: 20,
//     };
//     println!(
//         "here is the height of rect {} and here is the width {}",
//         rect.height, rect.width
//     );
//     let area = rect.area();
//     println!("the area of this rect is {}", area);
// }

// error 5

// Create a Rectangle struct with width and height
// Implement an area() method and a square constructor
// struct Rectangle {
//     // Your code here
// }

// impl Rectangle {
//     // Your code here
// }

// fn main() {
//     let rect = Rectangle::new(30, 50);
//     println!("Area: {}", rect.area());

//     let square = Rectangle::square(10);
//     println!("Square area: {}", square.area());
// }

// fix :-
// Create a Rectangle struct with width and height
// Implement an area() method and a square constructor
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn new(width: u32, height: u32) -> Rectangle {
//         Rectangle { width, height }
//     }

//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn square(size: u32) -> Rectangle {
//         Rectangle {
//             width: size,
//             height: size,
//         }
//     }
// }

// fn main() {
//     let rect = Rectangle::new(30, 50);
//     println!("Area: {}", rect.area());

//     let square = Rectangle::square(10);
//     println!("Square area: {}", square.area());
// }

// Enums (Like JS Union Types)
// Enums let you define a type by enumerating its possible values.

// enum TrafficLight {
//     Red,
//     Yellow,
//     Green,
// }

// fn main() {
//     let light = TrafficLight::Red;

//     match light {
//         TrafficLight::Red => println!("Stop!"),
//         TrafficLight::Yellow => println!("Slow down!"),
//         TrafficLight::Green => println!("Go!"),
//     }
// }

// error 6

// enum WebEvent {
//     PageLoad,
//     KeyPress(char),
// }

// fn main() {
//     // 1. Create a PageLoad event and handle it
//     // 2. Create a KeyPress event with 'a' and handle it
// }

// fix:-

// enum WebEvent {
//     PageLoad,
//     KeyPress(char),
// }

// fn main() {
//     // 1. Create a PageLoad event and handle it
//     // 2. Create a KeyPress event with 'a' and handle it
//     let loaded = WebEvent::PageLoad;
//     match loaded {
//         WebEvent::PageLoad => println!("Google help"),
//         WebEvent::KeyPress(c) => println!("here is the pressed char {}", c),
//     }

//     let key_pressed = WebEvent::KeyPress('a');
//     match key_pressed {
//         WebEvent::PageLoad => println!("Google help"),
//         WebEvent::KeyPress(c) => println!("here is the pressed char {}", c),
//     }
// }

// Define an enum called Shape
enum Shape {
    Circle(f64),         // Variant with associated data (radius)
    Square(f64),         // Variant with associated data (side length)
    Rectangle(f64, f64), // Variant with associated data (width, height)
}

// Function to calculate area based on the shape
fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side_length) => side_length * side_length,
        Shape::Rectangle(width, height) => width * height,
    }
}

fn main() {
    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);
    println!("This is the value of pi {}", std::f64::consts::PI);
    // Calculate and print the areas
    println!("Area of circle: {}", calculate_area(circle));
    println!("Area of square: {}", calculate_area(square));
    println!("Area of rectangle: {}", calculate_area(rectangle));
}
