use std::vec;

// vector is growable array/stucture
fn main() {
    let mut vector: Vec<i32> = vec![1, 2, 3];
    let _vector_string: Vec<String> = Vec::new();
    vector.push(4);
    vector.pop();
    vector.splice(0..=1, 3..=5);
    println!("vector: {:?}", &vector);

    let _third_element: &i32 = &vector[2]; // this is get the value at index
    let second_element: &Option<&i32> = &vector.get(1); // this is get index in Some

    match second_element {
        Some(val) => println!("second element: {}", val),
        None => println!("second element: None"),
    }


}
