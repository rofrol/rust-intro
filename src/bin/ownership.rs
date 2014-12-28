fn main() {
    println!("Hello, world!");
    let mut v = vec![];
    v.push("Hello");
    let p = v[0].clone();
    v.push("world");
    println!("{}", p);
}
