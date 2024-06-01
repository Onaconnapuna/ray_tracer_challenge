struct Tuple(f64, f64, f64, f64);

trait Point {
    fn is_point(&self) -> bool;
}

impl Point for Tuple { 
    fn is_point(&self) -> bool {
        if self.3 == 1.0 { return true } else { return false };
    }
}

trait Vector {
    fn is_vector(&self) -> bool;
}

impl Vector for Tuple { 
    fn is_vector(&self) -> bool {
        if self.3 == 0.0 { return true } else { return false };
    }
}
fn main() {
    let a_point = Tuple(4.3, -4.2, 3.1, 1.0);
    let a_vector = Tuple(4.3, -4.2, 3.1, 0.0);
    println!("a point is a Point: {}", a_point.is_point());
    println!("a point is a Vector: {}", a_point.is_vector());
    println!("a vector is a Point: {}", a_vector.is_point());
    println!("a vector is a Point: {}", a_vector.is_vector());
}
