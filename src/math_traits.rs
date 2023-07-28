pub trait CrossProduct {
    type Output;

    fn cross(self, _rhs: Self) -> Self::Output;
}

pub trait InnerProduct {
    type Output;

    fn dot(self, _rhs: Self) -> Self::Output;
}
