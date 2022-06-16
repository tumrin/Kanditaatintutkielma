let x = 0; // This is not mutable
let mut y = 0; // This is mutable
x = 1; // This causes compiler error
y = 1; // This is ok

// Both of these are correct
let x = 0;
let x: i32 = 0;