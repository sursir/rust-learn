
#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    pub fn intval(&self) -> Point<i32> {
        Point {
            x: unsafe {self.x.to_int_unchecked()},
            y: unsafe {self.y.to_int_unchecked()},
        }
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointX<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointX<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointX<X2, Y2>) -> PointX<X1, Y2> {
        PointX {
            x: self.x,
            y: other.y,
        }
    }
}


pub fn max_numbers(number_list: &Vec<i32>) {
    let mut largest = number_list[0];

    for &number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    // assert_eq!(largest, 100);
}

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
