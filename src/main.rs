fn main() {
    println!("{}, {}", "Hello", "World");
    println!("First Line\nSecond Line\nThird Line");
    //All content below is displayed as new line. If we skip the \ (Backslash) then the text will be displayed as is with all the spaces and newline characters.
    println!(
        "{}",
        "This \
    is \
    just \
    one \
    line."
    );
}
