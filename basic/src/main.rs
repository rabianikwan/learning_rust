fn main() {
    let a: u32 = 10; // immutable
    let mut b: u32 = 22; // mutable

    const HIGH_SCORE: u32 = 1_000_000; // const must be uppercase and defined type

    // shadowing
    let _x = 10;
    let x = "ten";
    println!("{:?}", x);// x changes become 10

    // types : integer ; floating_number ; characters ; boolean ;
    let _is_valid: bool = true;
    let _decimal: f32 = 3.14;
    let _char: char = 'a'; // char using single quotes
    let _string: String = String::from("hello");

    // compound/grouping types: tuples & arrays
    // tuple cannot be mutated size is fixed
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, _y, _z) = tup; // destructuring
    println!("tup: {:?}", x);

    let tup2 = (); // unit type value is unit value
    println!("tup2: {:?}", tup2);

    // array is also immutable and cannot be different type other than declared type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr: {:?}", arr);

    let arr2 = [3, 5]; // => let b = [3, 3, 3, 3, 3];
    println!("arr2: {:?}", arr2);

    my_function("rabian", 29);
    let func5 = valid_number(a, b);
    println!("{}", func5);

    // control flow / conditional
    // if else
    let control_flow_num = 10;
    if control_flow_num == 10 {
        println!("condition is true");
    } else {
        println!("condition is false")
    }

    // loop
    // this is loop that never stop except break
    loop {
        b -= 1;
        if b == 0 {
            break;
        }
        println!("infinite loop: b = {}", b);
    }

    // while loop
    while b < 20 {
        b += 1;
        println!("while loop: b = {}", b);
    }

    // for loop
    for element in arr {
        let new_element = element * 2;
        println!("for loop: {}", new_element);
    }
    println!("{:?}", arr);
}

// function with simple args
fn my_function(name: &str, age: u8) {
    println!("Hello, my name is {}, and my age is {}", name, age);
}
// func with defined result
fn valid_number(x: u32, y: u32) -> u32 {
    return x % y;
}
