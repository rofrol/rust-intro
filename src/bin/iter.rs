fn main() {
    println!("Hello iter");

    let v = vec![0u, 1u, 2u, 3u];

    for i in range(0u, v.len()) {
        println!("{}", v[i]);
    }

    for x in v.iter() {
        println!("{}", x);
    }
}
