fn move_example(string: String) {
    println!("{}", string);
}
fn move_example_with_return(string: String) -> String {
    println!("{}", string);
    string
}

fn main(){
    let string = String::from("Hello world");
    // Function takes ownership of string but returns it after running
    let string = move_example_with_return(string);
    println!("{}", string); // This is ok
    // This moves ownership to move_example function
    move_example(string); 
    println!("{}", string); // Compiler error!
}