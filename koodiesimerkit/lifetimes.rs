// Requires lifetime annotations
fn return_reference<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    // Compare lengths and return longer string slice
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}