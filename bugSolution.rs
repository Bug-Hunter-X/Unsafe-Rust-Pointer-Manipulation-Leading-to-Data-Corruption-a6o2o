fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of raw pointers, use safe methods to modify vector elements.
    v[0] = 10; // Safe and efficient way to modify the first element.
    println!("Modified vector: {:?}", v);
}