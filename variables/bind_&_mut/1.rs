fn main() {
    let x: i32 = 5; 
    let _y: i32; // Uninitialized but also unused, only a Warning ! There are two ways to fix this, one is by having _varname

    assert_eq!(x, 5);
    println!("Success!");
}