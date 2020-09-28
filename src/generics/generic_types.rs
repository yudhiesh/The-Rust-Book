// Placed inside the main function
//let p1 = generic_types::Point { x: 5, y: 0.2 };
//let p2 = generic_types::Point {
//    x: "Bye",
//    y: "Hello",
//};
//println!("p1 : {:#?}", p1);
//println!("p2 : {:#?}", p2);
//let result = p1.mixup(p2);
//println!("Mixup of is : {:#?}", result);
//println!("x : {:?} y : {:?}", result.x, result.y);

pub fn largest(list: &[i32]) -> &i32 {
    let mut _largest = &list[0];

    for number in list {
        if number > _largest {
            _largest = number;
        }
    }
    _largest
}

#[derive(Debug)]
pub struct Point<T, U> {
    pub x: T,
    pub y: U,
}

impl<T, U> Point<T, U> {
    pub fn mixup<V, W>(self, other: Point<V, W>) -> Point<V, U> {
        Point {
            x: other.x,
            y: self.y,
        }
    }
}
