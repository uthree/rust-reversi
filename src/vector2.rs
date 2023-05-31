#[derive(Clone, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> std::ops::Add<Vector2<T>> for Vector2<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Vector2<T>;
    fn add(self, other: Vector2<T>) -> Self::Output {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> std::ops::Sub<Vector2<T>> for Vector2<T>
where
    T: std::ops::Sub<Output = T> + Copy,
{
    type Output = Vector2<T>;
    fn sub(self, other: Vector2<T>) -> Self::Output {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> std::ops::Mul<T> for Vector2<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Vector2<T>;
    fn mul(self, scaler: T) -> Self::Output {
        Vector2 {
            x: self.x * scaler,
            y: self.y * scaler,
        }
    }
}

impl<T> std::ops::Div<T> for Vector2<T>
where
    T: std::ops::Div<Output = T> + Copy,
{
    type Output = Vector2<T>;
    fn div(self, scaler: T) -> Self::Output {
        Vector2 {
            x: self.x / scaler,
            y: self.y / scaler,
        }
    }
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector2 { x, y }
    }
}
