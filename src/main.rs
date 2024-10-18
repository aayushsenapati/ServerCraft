include!("../src/test.rs");
fn main() -> std::io::Result<()>{
    match test() {
        Ok(()) => println!("Test passed"),
        Err(e) => {
            println!("Test failed: {}", e);
            return Err(e);
        },
    }
    Ok(())
}
