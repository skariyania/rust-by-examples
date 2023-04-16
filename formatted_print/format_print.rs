fn main() {
    println!("{} is arg 1, {} is arg 2. {} is arg 3", "Sahil", "Bob", "Anna");
    println!("{0} is arg 1, {1} is arg 2. {1} is arg 2", "Sahil", "Bob");

    println!("named args {arg1}", arg1="this is arg1 text");
    println!("binary   {:b}", 7);
    println!("{number: >2}", number=2);
    println!("{number:0>2}", number=2);
    println!("my name is {0} {} {0}", "bond");

    #[allow(dead_code)]
    struct Structure(i32);
    // println!("print struct {}", Structure(3));

    // let number: f64 = 1.0;
    // let width: usize = 5;
    // println!("{number:>width}$");
}