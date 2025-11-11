// 1. Instalasi Rust (jika belum ada)
// # Install Rust melalui rustup
// curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
// source ~/.cargo/env

// 2. Setup Project
// # Masuk ke direktori example
// cd example

// # Atau jika belum ada, buat dengan cargo
// cargo new example
// cd example

// File: src/main.rs
// Tutorial Lengkap Bahasa Pemrograman Rust


// 3. Running Commands
// # Compile dan run sekaligus
// cargo run

// # Atau compile terlebih dahulu
// cargo build

// # Kemudian run executable
// ./target/debug/rust_example

// 4. Commands Tambahan
// # Check syntax tanpa compile
// cargo check

// # Build untuk production (optimized)
// cargo build --release

// # Run release version
// ./target/release/rust_example

// # Format code
// cargo fmt

// # Lint code
// cargo clippy



use std::collections::HashMap;
use std::fmt;

fn main() {
    println!("=== TUTORIAL LENGKAP RUST ===\n");
    
    // 1. VARIABLES DAN MUTABILITY
    demo_variables();
    
    // 2. DATA TYPES
    demo_data_types();
    
    // 3. FUNCTIONS
    demo_functions();
    
    // 4. CONTROL FLOW
    demo_control_flow();
    
    // 5. OWNERSHIP DAN BORROWING
    demo_ownership();
    
    // 6. STRUCTS
    demo_structs();
    
    // 7. ENUMS
    demo_enums();
    
    // 8. COLLECTIONS
    demo_collections();
    
    // 9. ERROR HANDLING
    demo_error_handling();
    
    // 10. TRAITS
    demo_traits();
    
    // 11. PATTERN MATCHING
    demo_pattern_matching();
    
    // 12. CLOSURES DAN ITERATORS
    demo_closures_iterators();
    
    // 13. MODULES (contoh inline)
    demo_modules();
    
    println!("\n=== TUTORIAL SELESAI ===");
}

// 1. VARIABLES DAN MUTABILITY
fn demo_variables() {
    println!("1. === VARIABLES DAN MUTABILITY ===");
    
    // let: membuat variable immutable (tidak bisa diubah)
    let x = 5;
    println!("Variable immutable x: {}", x);
    
    // mut: membuat variable mutable (bisa diubah)
    let mut y = 10;
    println!("Variable mutable y (awal): {}", y);
    y = 15;
    println!("Variable mutable y (setelah diubah): {}", y);
    
    // Shadowing: mendefinisikan ulang variable dengan nama sama
    let z = 100;
    let z = z + 1;  // ini bukan mutasi, tapi shadowing
    let z = z * 2;  // shadowing lagi
    println!("Variable z setelah shadowing: {}", z);
    
    // Constants: nilai yang tidak pernah berubah
    const MAX_POINTS: u32 = 100_000;
    println!("Constant MAX_POINTS: {}", MAX_POINTS);
    
    println!();
}

// 2. DATA TYPES
fn demo_data_types() {
    println!("2. === DATA TYPES ===");
    
    // Integer types
    let a: i32 = 42;          // 32-bit signed integer
    let b: u64 = 123_456;     // 64-bit unsigned integer
    let c = 98_222;           // default: i32
    
    println!("Integer - i32: {}, u64: {}, default: {}", a, b, c);
    
    // Floating point
    let d = 2.0;              // default: f64
    let e: f32 = 3.0;         // 32-bit float
    
    println!("Float - f64: {}, f32: {}", d, e);
    
    // Boolean
    let f = true;
    let g: bool = false;
    
    println!("Boolean - f: {}, g: {}", f, g);
    
    // Character (Unicode)
    let h = 'z';
    let i = '😎';
    let j = '中';
    
    println!("Char - ASCII: {}, Emoji: {}, Unicode: {}", h, i, j);
    
    // Tuple: kumpulan nilai dengan tipe berbeda
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;      // destructuring
    println!("Tuple - x: {}, y: {}, z: {}", x, y, z);
    println!("Tuple access - first: {}", tup.0);
    
    // Array: kumpulan nilai dengan tipe sama dan ukuran tetap
    let arr = [1, 2, 3, 4, 5];
    let arr2: [i32; 3] = [10, 20, 30];
    let arr3 = [3; 5];        // [3, 3, 3, 3, 3]
    
    println!("Array - arr[0]: {}, arr2[1]: {}", arr[0], arr2[1]);
    println!("Array filled - arr3: {:?}", arr3);
    
    // String types
    let s1 = "Hello";         // string literal (&str)
    let s2 = String::from("World"); // String (heap-allocated)
    
    println!("String types - &str: {}, String: {}", s1, s2);
    
    println!();
}

// 3. FUNCTIONS
fn demo_functions() {
    println!("3. === FUNCTIONS ===");
    
    // Function tanpa parameter dan return value
    simple_function();
    
    // Function dengan parameter
    let result = add_numbers(5, 3);
    println!("5 + 3 = {}", result);
    
    // Function dengan multiple return values (tuple)
    let (sum, diff) = calculate(10, 4);
    println!("10 + 4 = {}, 10 - 4 = {}", sum, diff);
    
    // Function expression (statement vs expression)
    let x = {
        let y = 3;
        y + 1  // tidak ada semicolon, ini expression yang di-return
    };
    println!("Function expression result: {}", x);
    
    println!();
}

fn simple_function() {
    println!("Ini adalah function sederhana tanpa parameter");
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b  // return tanpa keyword 'return'
}

fn calculate(x: i32, y: i32) -> (i32, i32) {
    (x + y, x - y)  // return tuple
}

// 4. CONTROL FLOW
fn demo_control_flow() {
    println!("4. === CONTROL FLOW ===");
    
    // if-else
    let number = 6;
    if number % 4 == 0 {
        println!("{} habis dibagi 4", number);
    } else if number % 3 == 0 {
        println!("{} habis dibagi 3", number);
    } else if number % 2 == 0 {
        println!("{} habis dibagi 2", number);
    } else {
        println!("{} tidak habis dibagi 4, 3, atau 2", number);
    }
    
    // if sebagai expression
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Conditional assignment: {}", number);
    
    // loop (infinite loop)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 3 {
            break counter * 2;  // return value dari loop
        }
    };
    println!("Loop result: {}", result);
    
    // while loop
    let mut num = 3;
    while num != 0 {
        println!("Countdown: {}!", num);
        num -= 1;
    }
    println!("LIFTOFF!");
    
    // for loop dengan range
    for i in 1..4 {  // 1, 2, 3 (tidak termasuk 4)
        println!("For range: {}", i);
    }
    
    // for loop dengan array
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("Array element: {}", element);
    }
    
    // for dengan enumerate (index + value)
    for (index, value) in a.iter().enumerate() {
        println!("Index {}: {}", index, value);
    }
    
    println!();
}

// 5. OWNERSHIP DAN BORROWING
fn demo_ownership() {
    println!("5. === OWNERSHIP DAN BORROWING ===");
    
    // Ownership move
    let s1 = String::from("hello");
    let s2 = s1;  // ownership pindah ke s2, s1 tidak valid lagi
    // println!("{}", s1); // ERROR! s1 sudah di-move
    println!("Owned string: {}", s2);
    
    // Clone untuk copy explicit
    let s3 = String::from("world");
    let s4 = s3.clone();  // explicit copy
    println!("Original: {}, Cloned: {}", s3, s4);
    
    // Borrowing dengan references
    let s5 = String::from("borrowing");
    let len = calculate_length(&s5);  // borrow s5
    println!("String '{}' has length {}", s5, len);  // s5 masih valid
    
    // Mutable borrowing
    let mut s6 = String::from("hello");
    change_string(&mut s6);
    println!("Changed string: {}", s6);
    
    // Slice: reference ke bagian dari data
    let s = String::from("hello world");
    let hello = &s[0..5];    // slice dari index 0-4
    let world = &s[6..11];   // slice dari index 6-10
    let whole = &s[..];      // slice seluruh string
    
    println!("Slices - hello: {}, world: {}, whole: {}", hello, world, whole);
    
    println!();
}

fn calculate_length(s: &String) -> usize {
    s.len()  // s adalah reference, tidak memiliki ownership
}  // s keluar dari scope, tapi karena tidak memiliki ownership, tidak ada yang di-drop

fn change_string(s: &mut String) {
    s.push_str(", world!");
}

// 6. STRUCTS
fn demo_structs() {
    println!("6. === STRUCTS ===");
    
    // Regular struct
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someuser"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("User: {} ({})", user1.username, user1.email);
    
    // Struct dengan mutable
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("anotheremail@example.com");
    println!("Updated user email: {}", user2.email);
    
    // Struct update syntax
    let user3 = User {
        email: String::from("new@example.com"),
        username: String::from("newuser"),
        ..user1  // ambil sisa field dari user1
    };
    println!("New user with copied fields: {}", user3.username);
    
    // Tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Color RGB: ({}, {}, {})", black.0, black.1, black.2);
    
    // Unit struct (tidak punya field)
    let _unit = UnitStruct;
    
    // Method calls
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("Rectangle area: {}", rect.area());
    println!("Rectangle can hold square? {}", rect.can_hold(&Rectangle { width: 10, height: 40 }));
    
    // Associated function (seperti constructor)
    let square = Rectangle::square(20);
    println!("Square area: {}", square.area());
    
    println!();
}

// Struct definition
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit struct
struct UnitStruct;

// Struct dengan methods
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method (menggunakan &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // Associated function (tidak menggunakan self)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 7. ENUMS
fn demo_enums() {
    println!("7. === ENUMS ===");
    
    // Basic enum
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("IP versions: {:?}, {:?}", four, six);
    
    // Enum dengan data
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    match home {
        IpAddr::V4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("IPv6: {}", addr),
    }
    
    // Option enum (built-in)
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("Option values: {:?}, {:?}, {:?}", some_number, some_string, absent_number);
    
    // Working with Option
    let x: i32 = 5;
    let y: Option<i32> = Some(5);
    
    match y {
        Some(value) => println!("y has value: {}", value),
        None => println!("y has no value"),
    }
    
    // if let (concise pattern matching)
    if let Some(3) = some_number {
        println!("three");
    } else {
        println!("not three, got: {:?}", some_number);
    }
    
    println!();
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 8. COLLECTIONS
fn demo_collections() {
    println!("8. === COLLECTIONS ===");
    
    // Vector: dynamic array
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    println!("Vector: {:?}", v);
    
    // Vector dengan macro
    let v2 = vec![1, 2, 3];
    println!("Vector with macro: {:?}", v2);
    
    // Accessing vector elements
    let third: &i32 = &v[2];
    println!("Third element: {}", third);
    
    match v.get(2) {
        Some(third) => println!("Third element (safe): {}", third),
        None => println!("No third element"),
    }
    
    // Iterating vector
    print!("Vector iteration: ");
    for i in &v {
        print!("{} ", i);
    }
    println!();
    
    // Mutable iteration
    for i in &mut v {
        *i += 50;
    }
    println!("Modified vector: {:?}", v);
    
    // String (collection of characters)
    let mut s = String::new();
    s.push_str("foo");
    s.push('l');
    s.push_str(" bar");
    println!("Built string: {}", s);
    
    // String concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 moved, s2 borrowed
    println!("Concatenated: {}", s3);
    
    // String slicing
    let hello = "Здравствуйте";
    let s = &hello[0..4];  // first 4 bytes (careful with Unicode!)
    println!("String slice: {}", s);
    
    // HashMap: key-value pairs
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    println!("HashMap: {:?}", scores);
    
    // Accessing HashMap values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("Blue team score: {}", s),
        None => println!("Blue team not found"),
    }
    
    // Iterating HashMap
    for (key, value) in &scores {
        println!("Team {}: {}", key, value);
    }
    
    // Update HashMap
    scores.insert(String::from("Blue"), 25);  // overwrite
    scores.entry(String::from("Red")).or_insert(0);  // insert if not exists
    scores.entry(String::from("Blue")).or_insert(100);  // won't overwrite
    
    println!("Updated HashMap: {:?}", scores);
    
    println!();
}

// 9. ERROR HANDLING
fn demo_error_handling() {
    println!("9. === ERROR HANDLING ===");
    
    // Result enum untuk error handling
    let result = divide(10, 2);
    match result {
        Ok(value) => println!("10 / 2 = {}", value),
        Err(e) => println!("Error: {}", e),
    }
    
    let result2 = divide(10, 0);
    match result2 {
        Ok(value) => println!("10 / 0 = {}", value),
        Err(e) => println!("Error: {}", e),
    }
    
    // unwrap (panic if error)
    // let bad = divide(10, 0).unwrap();  // akan panic!
    
    // expect (panic dengan custom message)
    // let bad = divide(10, 0).expect("Division by zero!");
    
    // ? operator untuk error propagation
    match read_number_from_string("42") {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Parse error: {}", e),
    }
    
    match read_number_from_string("abc") {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Parse error: {}", e),
    }
    
    println!();
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

fn read_number_from_string(s: &str) -> Result<i32, std::num::ParseIntError> {
    let num: i32 = s.parse()?;  // ? akan return error jika parse gagal
    Ok(num)
}

// 10. TRAITS
fn demo_traits() {
    println!("10. === TRAITS ===");
    
    // Implement trait untuk struct
    let article = NewsArticle {
        headline: String::from("Breaking News!"),
        location: String::from("Indonesia"),
        author: String::from("John Doe"),
        content: String::from("Something important happened..."),
    };
    
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    println!("Article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());
    
    // Default implementation
    println!("Article default: {}", article.summarize_default());
    
    // Trait sebagai parameter
    notify(&article);
    notify(&tweet);
    
    // Trait bounds
    let pair = Pair::new(5, 10);
    pair.cmp_display();
    
    println!();
}

// Trait definition
trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation
    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Trait sebagai parameter
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bounds dengan generics
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 11. PATTERN MATCHING
fn demo_pattern_matching() {
    println!("11. === PATTERN MATCHING ===");
    
    // Basic match
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything else"),
    }
    
    // Match dengan multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3..=5 => println!("three through five"),
        _ => println!("anything else"),
    }
    
    // Destructuring structs
    let p = Point3D { x: 0, y: 7, z: 0 };
    match p {
        Point3D { x, y: 0, z: 0 } => println!("On the x axis at {}", x),
        Point3D { x: 0, y, z: 0 } => println!("On the y axis at {}", y),
        Point3D { x: 0, y: 0, z } => println!("On the z axis at {}", z),
        Point3D { x, y, z } => println!("On none of the axes: ({}, {}, {})", x, y, z),
    }
    
    // Match guards
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("got: {}", x),
        None => (),
    }
    
    // @ bindings
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
    
    println!();
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Hello { id: i32 },
}

// 12. CLOSURES DAN ITERATORS
fn demo_closures_iterators() {
    println!("12. === CLOSURES DAN ITERATORS ===");
    
    // Basic closure
    let expensive_closure = |num| {
        println!("calculating slowly...");
        std::thread::sleep(std::time::Duration::from_millis(100));
        num
    };
    
    println!("Result: {}", expensive_closure(5));
    
    // Closure dengan environment capture
    let x = 4;
    let equal_to_x = |z| z == x;  // capture x dari environment
    let y = 4;
    println!("y equals x? {}", equal_to_x(y));
    
    // Iterator methods
    let v1 = vec![1, 2, 3];
    
    // map: transform setiap element
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("Original: {:?}, Mapped: {:?}", v1, v2);
    
    // filter: filter elements
    let v3: Vec<_> = v1.iter().filter(|&x| *x > 1).collect();
    println!("Filtered (> 1): {:?}", v3);
    
    // fold/reduce
    let sum: i32 = v1.iter().sum();
    println!("Sum: {}", sum);
    
    // find
    let found = v1.iter().find(|&&x| x > 2);
    match found {
        Some(value) => println!("Found value > 2: {}", value),
        None => println!("No value > 2 found"),
    }
    
    // any/all
    let any_even = v1.iter().any(|&x| x % 2 == 0);
    let all_positive = v1.iter().all(|&x| x > 0);
    println!("Any even: {}, All positive: {}", any_even, all_positive);
    
    // enumerate
    for (index, value) in v1.iter().enumerate() {
        println!("Index {}: {}", index, value);
    }
    
    // zip
    let names = vec!["Alice", "Bob", "Carol"];
    let ages = vec![25, 30, 35];
    let combined: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("Combined: {:?}", combined);
    
    // Custom iterator adapter
    let doubled: Vec<_> = v1.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    
    println!();
}

// 13. MODULES (contoh inline)
fn demo_modules() {
    println!("13. === MODULES ===");
    
    // Call function dari module
    my_module::public_function();
    
    // Use nested module
    my_module::nested::nested_function();
    
    println!();
}

mod my_module {
    pub fn public_function() {
        println!("Called my_module::public_function()");
        private_function();
    }
    
    fn private_function() {
        println!("Called my_module::private_function()");
    }
    
    pub mod nested {
        pub fn nested_function() {
            println!("Called my_module::nested::nested_function()");
        }
    }
}
