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
