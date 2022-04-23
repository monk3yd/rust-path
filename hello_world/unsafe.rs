fn main()
{
    // Create address variable number that points to an undefined size chunck of memory 
    let address = 0x12345usize;  // Bad memory allocation, 0x12345 doesn't exist in memory

    // Use address number called it r1 and treated as a pointer const to interger 32
    let r1 = address as *const i32;

    // Use unsafe low level code e.g dereference pointer
    unsafe 
    {
        println!("r1 is {}", *r1);
    }

}