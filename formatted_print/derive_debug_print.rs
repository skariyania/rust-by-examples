fn main () {
    #[derive(Debug)]
    struct DebugPrintable(i32);

    #[derive(Debug)]
    struct Deep(DebugPrintable);

    println!("{:?} months", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "slater", "christian", actor="actor's");
    println!("{1} {0} is the {actor} name.", "slater", "christian", actor="actor's");
    println!("structures are also printable {:?}", DebugPrintable(3));
    println!("structures are also printable {:?}", Deep(DebugPrintable(3)));
}