// ******** to run a function just Change the Name of it to main ********
// ************** Only one main function can run at a time **************

// **********************************************************************
// **********************************************************************
// **********************************************************************
// **********************  Formatted Print ******************************
// **********************************************************************
// **********************************************************************
// **********************************************************************
// **********************************************************************

// use std::fmt;

// fn main0() {
//     // In general, the `{}` will be automatically replaced with any
//     // arguments. These will be stringified.
//     println!("Mitrooo {}", "this is the first Print");

//     // Positional arguments can be used. Specifying an integer inside `{}`
//     // determines which additional argument will be replaced. Arguments start
//     // at 0 immediately after the format string.
//     println!(
//         "{0}, this is {1}. {1}, this is {0}",
//         "Mr. Khan", "Mr. Nawaz Sharif"
//     );

//     // As can named arguments.
//     println!(
//         "{subject} {verb} {object}",
//         object = "the lazy dog",
//         subject = "the quick brown fox",
//         verb = "jumps over"
//     );

//     let x = 69420;
//     // Different formatting can be invoked by specifying the format character
//     // after a `:`.
//     println!(
//         "Base 10 (Nothing in the parenthesis for decimal):     `{}`",
//         x
//     ); // 69420
//     println!("Base 2  (':b' in the parenthesis for binary):   `{:b}`", x); // 10000111100101100
//     println!("Base 8  (':o' in the parenthesis for octal):    `{:o}`", x); // 207454
//     println!(
//         "Base 16 (':x' in the parenthesis for hexadecimal):  `{:x}`",
//         x
//     ); // 10f2c

//     // You can right-justify text with a specified width. This will
//     // output "    1". (Four white spaces and a "1", for a total width of 5.)
//     println!("`{number:>5}` by writing number:>5.", number = 1);
//     // You can Left-justify text with a specified width. This will
//     // output "1    ". (1 and Four white spaces, for a total width of 5.)
//     println!("`{number:<5}` by writing number:<5.", number = 1);

//     // You can pad numbers with extra zeroes,
//     println!("`{number:0>5}` by writing number:0>5", number = 1); // 00001
//     // and left-adjust by flipping the sign. This will output "10000".
//     println!("`{number:0<5}` by writing number:0<5", number = 1); // 10000

//     // You can use named arguments in the format specifier by appending a `$`.
//     println!(
//         "`{number:0>width$}` by writing number:0>width$",
//         number = 1,
//         width = 5
//     );

//     // Rust even checks to make sure the correct number of arguments are used.
//     // println!("My name is {0}, {1} {0}", "Bond"); // This Line has an error which is that it is missing an argument (1)
//     // FIXME ^ Add the missing argument: "James"
//     println!("My name is {0}, {1} {0}", "Bond", "James"); // This Line has an error which is that it is missing an argument (1)

//     // struct

//     // Only types that implement fmt::Display can be formatted with `{}`. User-
//     // defined types do not implement fmt::Display by default.
//     #[allow(dead_code)] // disable `dead_code` which warn against unused module
//     struct Structure(i8);

//     // This will not compile because `Structure` does not implement
//     // fmt::Display;
//     // println!("This struct `{}` won't print...", Structure(3));
//     // TODO ^ Try uncommenting this line
//     // This will not compile because `Structure` does not implement
//     impl fmt::Display for Structure {
//         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//             write!(f, "{0}, {1}", self.0, self.0 - 1)
//         }
//     }
//     let y = 127;
//     println!("This struct {1} Will print...  `{}`", Structure(y), y);

//     struct Greetings(String, String);

//     impl fmt::Display for Greetings {
//         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//             write!(f, "{} {}", self.0, self.1)
//         }
//     }

//     let z: String = "Good Morning".to_string();
//     let z1: String = "Nawaz".to_string();
//     println!("`{}`", Greetings(z, z1));

//     // For Rust 1.58 and above, you can directly capture the argument from a
//     // surrounding variable. Just like the above, this will output
//     // "    1", 4 white spaces and a "1".
//     let number: f64 = 1.0;
//     let width: usize = 5;
//     println!("{number:>width$}"); // 1 // No need to specify the type and the vars in printLn args
// }

// **********************************************************************
// **********************************************************************
// **********************************************************************
// ********************** Some Basic Syntax *****************************
// **********************************************************************
// **********************************************************************
// **********************************************************************
// **********************************************************************

// fn main1() {
//     // Loops and Iterators
//     for i in 0..10 {
//         println!("The number is {}", i);
//     }

//     // if else statement
//     let _is_above_18 = true;
//     let is_male = false;
//     if !is_male {
//         println!("Female");
//     } else {
//         println!("Male");
//         print!("Male ");
//         print!("{}", !is_male);
//     }
//     if _is_above_18 && is_male {
//         println!("You are above 18 and male");
//     } else if _is_above_18 && !is_male {
//         println!("You are above 18 and female");
//     } else {
//         println!("You are not above 18 and male");
//     }

//     // MATCH
//     let greetings = "Hello";
//     match greetings {
//         "Hello" => println!("English"),
//         "Halo" => println!("Indonesia"),
//         _ => println!("Unknown"),
//     }
//     let char1 = greetings.chars().nth(2);

//     match char1 {
//         Some(c) => println!("Hello Buddy here is the char {}", c),
//         None => println!("None"),
//     }
// }

// **********************************************************************
// **********************************************************************
// **********************************************************************
// ************************* MATCH **************************************
// **********************************************************************
// **********************************************************************
// **********************************************************************
// **********************************************************************

// fn main2() {
//     let number = 13;
//     // TODO ^ Try different values for `number`

//     println!("Tell me about {}", number);
//     match number {
//         // Match a single value
//         1 => println!("One!"),
//         // Match several values
//         2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
//         // TODO ^ Try adding 13 to the list of prime values
//         // Match an inclusive range
//         13..=19 => println!("A teen"),
//         // Handle the rest of cases
//         _ => println!("Ain't special"),
//         // TODO ^ Try commenting out this catch-all arm
//     }

//     let boolean = true;
//     // Match is an expression too
//     let binary = match boolean {
//         // The arms of a match must cover all the possible values
//         false => 0,
//         true => 1,
//         // TODO ^ Try commenting out one of these arms
//     };

//     println!("{} -> {}", boolean, binary);
// }

// **********************************************************************
// **********************************************************************
// **********************************************************************
// ***************************** String *********************************
// **********************************************************************
// **********************************************************************
// **********************************************************************
// **********************************************************************

// Strings (I Will Do it Soon)

// &str
// String
// Arc<str>

/// This function will cover the String part of Rust
/// It will have the details about
/// - How to create a String
/// - How to Index a String
/// - How to iterate over a String
/// - How to split a String
/// - How to Concatenate Strings
/// - How to Format Strings

fn main() {
    // Will do the String Part here
    let mut s = String::new();
    s.push_str("Hello");
    s.push(' ');
    s.push_str("World");
    println!("{}", s);

    // &str
    let hello = "Hello";
    let hello = &hello[0..4];
    println!("{}", hello);
}
