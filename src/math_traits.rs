pub trait CrossProduct {
    type Output;

    fn cross(&self, _rhs: &Self) -> Self::Output;
}

pub trait InnerProduct {
    type Output;

    fn dot(&self, _rhs: &Self) -> Self::Output;
    fn length_squared(&self) -> Self::Output;
    fn length(&self) -> Self::Output;
    fn unit(&self) -> Self;
}
