fn main() {
    println!("Types in Rust!");

    let my_i8: i8 = 42;
    println!("i8: {:?}", my_i8);
    
    let my_i128: i128 = 4242424242424242;
    println!("i128: {:?}", my_i128);

    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);

    assert_eq!((-128_i8).checked_div(-1), None);

    println!("{}", (32760_i16.saturating_add(10)));
    println!("{}", (32760_i16.wrapping_add(10)));
    println!("{}", (32760_i16.overflowing_add(10).0));

}
