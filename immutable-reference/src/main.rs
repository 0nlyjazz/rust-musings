fn main() {
    let y = &mut 42;
    println!("y = {}",y);
    *y = 56;
    println!("y = {}",y);

    let z = &y;
    **z = 109;
    println!("z = {}",z);
}
