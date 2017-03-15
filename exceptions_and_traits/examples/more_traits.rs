use std::fmt;

#[derive(Debug)]
struct GenPoint<T> {
    x: T, y: T,
}

impl<T: fmt::Display> fmt::Display for GenPoint<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "GenPoint {{ x: {}, y: {} }}", self.x, self.y)
    }
}


fn main() {
    let point = GenPoint {x: 23, y: 1 };

    println!("{}", point);
    println!("{:?}", point);
}
