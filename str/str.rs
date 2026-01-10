fn main() {
    let _truthy_str: &str = "Hello, Rust!";
    let zen: &'static str = "The Zen of Rust.";
    let ptr = zen.as_ptr();
    let len = zen.len();
    assert_eq!(len, 16);

    let s = unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8(slice)
    };
    assert_eq!(s, Ok(zen));

    // basic string operations
    let hello = "Hello, ";
    let world = "world!";
    let hello_world = format!("{}{}", hello, world);
    assert_eq!(hello_world, "Hello, world!");
    assert_eq!(hello_world.len(), 13);
    assert_eq!(hello_world.is_empty(), false);
    assert_eq!(hello_world.contains("world"), true);
    assert_eq!(hello_world.replace("world", "Rust"), "Hello, Rust!");
    let substring = &hello_world[7..12];
    assert_eq!(substring, "world");

    // split operation
    let csv = "apple,banana,cherry";
    let fruits: Vec<&str> = csv.split(',').collect();
    assert_eq!(fruits, vec!["apple", "banana", "cherry"]);

    // mutable string
    let mut mutable_string = String::from("Hello");
    mutable_string.push_str(", Rust!");
    assert_eq!(mutable_string, "Hello, Rust!");

    // concatenation
    let s1 = String::from("Good ");
    let s2 = String::from("Morning");
    let s3 = s1 + &s2; // s1 is moved here and
    assert_eq!(s3, "Good Morning");

    // iteration over characters
    let sample = "Rust";
    let chars: Vec<char> = sample.chars().collect();
    assert_eq!(chars, vec!['R', 'u', 's', 't']);
    
}