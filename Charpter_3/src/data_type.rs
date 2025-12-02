fn main() {
    // Integers

    let a = 98_222; // Decimal
    let b = 0xff; // Hexadecimal
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8 only)

    let f: u8 = 255; // Explicit type annotation

    // Booleans
    let t = true;

    let f: bool = false; 

    // Characters

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // Compund Types
    let tup = ("Let's Get Rusty!", 100_000);
    let (channel, sub_count) = tup;
    
}