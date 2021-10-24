fn main() {
    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);
    let x = 5;
    println!("shadow s : {}", x);

    let x = x + 1;
    println!("shadow s : {}", x);
}
