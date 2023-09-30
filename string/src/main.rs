fn main() {
    // this is how to create a string
    // kedua adalah hal yang sama
    let data = "ayam goreng".to_string();
    let data_ke2 = String::from("ayam goreng");

    let data_ke3 = data + " " + &data_ke2;
    println!("{}", data_ke3);
}
