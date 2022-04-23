fn main()
{
    // Create address variable number that points to an undefined size
    let address = 0x12345usize;
    let r1 = address as *const i32

    // Declare safe code variable
    let x: u32 = 4;

    // In C an illegal access to memory could be
    // &x[4]

    // Use unsafe low level code e.g dereference pointer
    unsafe {

    }

    println!("Hello, World!, {}", x);
}