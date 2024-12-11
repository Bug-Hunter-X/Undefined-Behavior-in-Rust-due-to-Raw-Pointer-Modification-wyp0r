fn main() {
    let mut v = vec![1, 2, 3];
    //Instead of using raw pointers, directly modify the vector
    v[0] = 10;
    println!("{:?}", v);
}