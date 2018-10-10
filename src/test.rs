#[test]
fn test_struct() {
    let d = Desktop::get();
    println!("{}", d.environment());
}