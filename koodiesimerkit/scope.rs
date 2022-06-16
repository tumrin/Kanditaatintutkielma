{
    // Create string. This allocates memory from heap
    let string = String::from("Hello world"); 
    println!("{}", string); // Prints string
} // String is dropped here because after this it is out of scope

 // This causes compilation error since string has been dropped
println!("{}", string);

{
    // Box can be used to allocate memory from heap for any type
    let boxed_int = Box::new(10);
} // boxed_int is no longer valid after this block ends
