mod lib;
fn main() {
    let num = 10;
    let num2 = 15;

    println!("{} + {} = {}", num, num2, lib::add(num, num2));
    println!("{} - {} = {}", num, num2, lib::sub(num, num2));
    println!("{} * {} = {}", num, num2, lib::mul(num, num2));
}