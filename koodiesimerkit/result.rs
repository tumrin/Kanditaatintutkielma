fn read_file() -> Result<String, Error> {
    let file_content = fs::read_to_string("./textfile.txt");
    file_content
}
fn main() {
    // Match return value. Match must be exhaustive i.e all possible cases must be handled
    let string = match read_file() {
        Ok(content) => content,
        Err(error) => {
            // Print error message
            println!("Encountered {error} while reading file");
            // Set string variable to error message
            "Error reading file content".to_string()
        }
    };
    println!("{}", string);
}