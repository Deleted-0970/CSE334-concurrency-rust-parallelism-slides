//! Run this file with `cargo test --test 03_struct`.

// TODO: Implement a `Vec3` structure that represents a three-dimensional vector.
// Use field names `x`, `y` and `z`.
// Implement `new`, `add`, `length` and `normalize` methods, so that tests pass.
// If you `normalize` a vector that has length 0, it should return a zero-length vector.
struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vec3<T>
where
    T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + std::ops::Div<Output = T>,
    T: Clone + Copy,
    T: Default,
    T: std::cmp::PartialEq,
    T: From<f64>,
    T: Into<f64>,
{
    fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
    //fn hi(self: Self, n: i32) { }
    /*
    fn add(&mut self, o: Vec3) {
        self.x += o.x;
        self.y += o.y;
        self.z += o.z;
    }
    */
    fn add(&self, o: Vec3<T>) -> Self {
        Vec3 {
            x: self.x + o.x,
            y: self.y + o.y,
            z: self.z + o.z,
        }
    }
    fn length(&self) -> T {
        return T::from(
            ((self.x * self.x + self.y * self.y + self.z * self.z).into() as f64).sqrt(),
        );
        //return (self.x * self.x + self.y * self.y + self.z * self.z)
    }
    fn normalize(&self) -> Self {
        let l = self.length();
        if l == T::default() {
            Vec3 {
                x: T::default(),
                y: T::default(),
                z: T::default(),
            }
        } else {
            Vec3 {
                x: self.x / l,
                y: self.y / l,
                z: self.z / l,
            }
        }
    }
}
/*
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

struct Vec3(f64, f64, f64);

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }
    //fn hi(self: Self, n: i32) { }
    /*
    fn add(&mut self, o: Vec3) {
        self.x += o.x;
        self.y += o.y;
        self.z += o.z;
    }
    */
    fn add(&self, o: Vec3) -> Self {
        Vec3 {
            x: self.x + o.x,
            y: self.y + o.y,
            z: self.z + o.z,
        }
    }
    fn length(&self) -> f64 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    fn normalize(&self) -> Self {
        let l = self.length();
        if l == 0.0 {
            Vec3 {x: 0.0, y: 0.0, z: 0.0}
        } else {
            Vec3 {x: self.x / l, y: self.y / l, z: self.z / l}
        }
    }
}
*/

/*
fn main() {
    let x = Vec3::new(1,2,3);
    x.hi(1); // Vec3::hi(x, 1);
    x.hi(2); // Vec3::hi(x, 2);

}
*/

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn new() {
        let v1 = Vec3::new(1.2, 3.5, 6.0);
        assert_eq!(v1.x, 1.2);
        assert_eq!(v1.y, 3.5);
        assert_eq!(v1.z, 6.0);
    }

    #[test]
    fn add() {
        let v1 = Vec3::new(1.2, 3.5, 6.0);
        let v2 = Vec3::new(4.8, 6.2, -2.3);
        let v3 = v1.add(v2);
        assert_almost_eq(v3.x, 6.0);
        assert_almost_eq(v3.y, 9.7);
        assert_almost_eq(v3.z, 3.7);
    }

    #[test]
    fn length_zero() {
        assert_eq!(Vec3::new(0.0, 0.0, 0.0).length(), 0.0);
    }

    #[test]
    fn length() {
        assert_almost_eq(Vec3::new(-6.2, 13.85, 12.8).length(), 19.852);
    }

    #[test]
    fn normalize_zero() {
        let norm = Vec3::new(0.0, 0.0, 0.0).normalize();
        assert_eq!(norm.x, 0.0);
        assert_eq!(norm.y, 0.0);
        assert_eq!(norm.z, 0.0);
    }

    #[test]
    fn normalize() {
        let norm = Vec3::new(1.5, 28.4, -5.6).normalize();
        assert_almost_eq(norm.x, 0.051);
        assert_almost_eq(norm.y, 0.98);
        assert_almost_eq(norm.z, -0.1932);
    }

    #[track_caller]
    fn assert_almost_eq(value: f64, expected: f64) {
        assert!(
            (value - expected).abs() < 0.001,
            "{value} does not equal {expected}"
        );
    }
}
