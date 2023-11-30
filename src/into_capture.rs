use crate::capture::Capture;

pub trait IntoCapture
where
    Self: Sized,
{
    fn capture(self) -> Capture<Self>;
}

impl<T> IntoCapture for T {
    fn capture(self) -> Capture<Self> {
        Capture(self)
    }
}
