
struct Point {
    x: f64,
    y: f64,
}

struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

/// Method ==== Start ====
/// Implement the Distance trait for Point and Vector
trait Distance {
    fn distance_from_origin(&self) -> f64;
}
impl Distance for Point {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
impl Distance for Vector {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}
/// Method ==== End ====

/// Macro ==== Start ====
/// Macro to implement the distance_from_origin method for Point and Vector
/// Macro ==== End ====
// This macro requires nightly Rust
macro_rules! distance_from {
    ($head:expr, $($tail:expr),+) => {
      $head + distance_from!($($tail),+)
    };
    ($head:expr) => {
      $head
    };
  }
  
fn main() {
  let result = sum_of_squares(1, 2);
  println!("Sum: {}", result); // Output: Sum: 15
}
  
#[cfg(not(feature = "fake_result"))]
fn sum_of_squares(x: i32, y: i32) -> i32 {
    x * x + y * y
}

#[cfg(feature = "fake_result")]
fn sum_of_squares(x: i32, y: i32) -> i32 {
    x * x + y * y
}