fn main() {
    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);

    assert_eq!((-128_i8).checked_div(-1), None);

    println!("{}", (32760_i16.saturating_add(10)));
    println!("{}", (32760_i16.wrapping_add(10)));
    println!("{}", (32760_i16.overflowing_add(10).0));

}
