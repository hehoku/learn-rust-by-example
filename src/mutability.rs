#[allow(dead_code)]
pub fn test() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // 会报错
    // _immutable_binding += 1;
}
