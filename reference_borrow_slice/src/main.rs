fn main() {
    // reference is borrowing a value then reference = borrowing, is not taken ownership
    // if finish we give back into the original ownership
    // reference is as default immutable, is just like borrowing something we give back
    // to the ownership what we borrowed
    let string_reference = String::from("hello");
    let string_len = calculate_length(&string_reference); // using & is for reference
    println!("The length of '{}' is {}.", string_reference, string_len);
    
    // this is example of how to mutable using reference
    let mut string: String = String::from("hello");
    greetings(&mut string);
    println!("Greeting: {}", string);
    
    // WE CAN ONLY have 1 mutable reference at a time
    let r1 = &mut string;
    //    let r2 = &mut string; This second mutable reference is not allowed
    
    let mut not_allow = String::from("hello");
    let x1 = &not_allow;
    let x2 = &not_allow;
    // let x3 = &mut not_allow; // this will also causing error because x1 and x2 is immutable
    println!("{}, {}", x1, x2);
    
    // how to borrow mutable in this cases?
    // this is allowed because x1 and x2 has been finished using and give back
    // the value to not_allow then, x3 using as reference become mutable
    // this way is allowed
    let x3 = &mut not_allow;
    println!("{}", x3);
    
    // ini adalah string slice
    // string slice adalah cara memasukan string ke dalam fungsi tanpa khawatir fungsi tersebut
    // mengambil alih ownership dari string
    
    let mut sample = String::from("hello world");
    let hello: &str = &sample[..=5];
    let world: &str = &sample[6..];
    println!("{} {}", hello, world);
    let word = first_word(&sample);
    println!("{}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item ) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
fn calculate_length(string: &String) -> usize {
    string.len()
}

fn greetings(string: &mut String) {
    string.push_str(", master!");
}