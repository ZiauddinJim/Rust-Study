const GLOBAL_VALUE: u8 = 50;
fn main() {
    let outside_value = 5;
    {
        let inside_value = 7;
        println!("Inside variable: {inside_value}");
        println!(" outside variable: {outside_value}");
    }
    
    println!(" outside variable: {outside_value}");
    print_value();
}

fn print_value() {
    println!("Global Value: {GLOBAL_VALUE}")
}
