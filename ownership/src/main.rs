// ownership is rust feature to manage stack and heap memory
fn main() {
	{
		// this is line cannot use string_scope because it is not yet declared
		let mut string_scope: String= String::from("hello");
		// string_scope can be used this line further
		string_scope.push_str(", world");
		println!("{}", string_scope);
	}
	// string_scope is dropped because out of scope
	
	// copy trait is for a integer, array, and tuple with no string in the reference
	let x = 5;
	let y = x; 
	println!("x: {} have been copy by y: {}", x, y);
	
	// move trait is happen in string, the value is moved and original variable cannot be used anymore
	let string_original = String::from("hello");
	let string_moved = string_original;
	let string_copy = string_moved.clone();
	println!("string original move the ownership to string_moved: {}, then string copy clone it {}", string_moved, string_copy);
	
	let s: String = String::from("Hello");
	takes_ownership(s); // s values move to function
	
	let number = 5;
	makes_copy(number); // number value copy to function
	println!("{}", number); // number still can be used because of copy
	
	let string2 = String::from("is it moved");
	let string3: String = take_and_giveback(string2);
	println!("{}", string3); // it also moved except we use clone() function
}

fn take_and_giveback(string: String) -> String {
	string
}
fn takes_ownership(some_string: String) {
	println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
	println!("{}", some_integer);
}
