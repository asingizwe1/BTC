struct Point<T> {
    x: T,
    y: T,
}

impl Point {
    //can every type be multiplied - hence trait bounds (what must be supported)
    //mul,add, does u8 have sqrt-no neither does i32 but f32 does
    fn distance_from_zero() {}
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
        //basing
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
