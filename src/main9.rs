use std::ops::{Add, Mul};
//add trait bounds to struct
struct Point<T: Add + Mul> {
    x: T,
    y: T,
}

//since its generic it should also be defined for the impl block
//+ is used to combine multiple trait bounds on the same type.
impl<T> Point<T>
where
    // "T must implement Add, Mul, and Copy."
    T: Add<Output = T> + Mul<Output = T> + Copy,
    f64: From<T>, //
{
    //can every type be multiplied - hence trait bounds (what must be supported)
    //mul,add, does u8 have sqrt-no neither does i32 but f32 does
    //trait bounds as guarantees that certain methods are allowed to be called.
    //we need to guarantee that the type passed in can be added to, multiplied to, copied out of self without moving, and converted to the f64 type.
    fn distance_from_zero(&self) -> f64 {
        let float_sum: f64 = (self.x * self.x + self.y * self.y).into();
        float_sum.sqrt()
    }
}

fn main() {
    let p1 = Point { x: 3_u8, y: 4_u8 };
    println!("distance: {}", p1.distance_from_zero());

    let p2 = Point { x: 3_f32, y: 4_f32 };
    println!("distance: {}", p2.distance_from_zero());

    let p3 = Point { x: -3, y: -4 };
    println!("distance: {}", p3.distance_from_zero());

    // this should not be allowed
    // let p4 = Point { x: "a", y: "b" };
}

#[cfg(test)]
mod unit_tests {
    use super::Point;

    #[test]
    fn test_point_u8() {
        let p1 = Point { x: 3_u8, y: 4_u8 };
        //basing on this comparison we need a trait that allows conversion ie Into,From
        assert_eq!(p1.distance_from_zero(), 5.0);
    }

    #[test]
    fn test_point_f32() {
        let p2 = Point { x: 3_f32, y: 4_f32 };
        assert_eq!(p2.distance_from_zero(), 5.0);
    }

    #[test]
    fn test_point_i32() {
        let p3 = Point {
            x: -3_i32,
            y: -4_i32,
        };
        assert_eq!(p3.distance_from_zero(), 5.0);
    }
}
