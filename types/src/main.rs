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

    assert_eq!(5_f32.sqrt() * 5_f32.sqrt(), 5.);
    assert_eq!((-1.01_f64).floor(), -2.0);

    println!("{}", (2.0_f64).sqrt());

    assert_eq!('*' as i32, 42);

    // Tuples
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    // Pointer Types
    let t = (12, "eggs");
    let b = Box::new(t);

    // Arrays
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    assert!(sieve[211]);
    assert!(!sieve[9876]);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}
