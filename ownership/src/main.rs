// fn main() {
//     let   str1 = String::from("Hello");
//     // str1.push_str(" World");
//     let str2 = str1;
//     println!("{str2}");
//     // println!("{str2}")
// }

// /* Note:  this code working
// let a = 5;
//     let b = a;
//     println!("a = {a}");
//     println!("b = {b}");
// */
// Note: Function dia ownarship practice (Stack memory)
// fn main() {
//     let x: u8 = 5; //  x memory 5
//     process_integer(x);
//     println!("the value of x in main(): {x}");
// }

// fn process_integer(item: u8) { // item memory of 5
//     println!("The value of item in process_integer(): {item}");
// }

// Note: Heap memory
// fn main() {
//     let x: String = String::from("Hello"); // x is the owner of Hello
//     process_integer(x); // transfer of ownership
//     // println!("the value of x in main(): {x}");
// }

// fn process_integer(item: String) {
//     //Hello new owner is item
//     println!("The value of item in process_integer(): {item}");
// }

fn main() {
    let s1: String = get_string(); // Note: Ownership get_string move in the S1
    println!("This is S1: {s1}");

    let s2: String = String::from("World"); // Note: world main ownership in the s2
    let s3: String = send_get_string(s2); // Note: Final ownership s3

    println!("This is s3: {s3}")
}

fn get_string() -> String {
    // Note: Hello main ownership get_String
    let new_string = String::from("Hello");
    return new_string;
}

fn send_get_string(recive_string: String) -> String {
    // Note: new ownership world send_get_string fn
    return recive_string;
}
// Documentation:
// Hello ownership get_string -> s1
// World ownership s2 -> send_get_string -> s3
