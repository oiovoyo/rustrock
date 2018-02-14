// shadowing can change type of x
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    let x = x / 3;
    println!("x is : {}", x);
}
