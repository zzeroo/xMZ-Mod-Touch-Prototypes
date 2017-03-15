fn min<T>(a: T, b: T) -> T
    where T: PartialOrd
{
    if a < b { a } else { b }
}


fn main() {
    println!("was ist kleiner? 99 oder 23");
    println!("{}", min(99, 23));
    println!("Geht auch mit Floats: 33.4 und 123.9");
    println!("{}", min(33.4, 123.9));
    println!("Oder mit str? 'zzeroo' und 'KLS'");
    println!("{}", min("zzeroo", "KLS"));
}
