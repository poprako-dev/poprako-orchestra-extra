use std::time::Duration;

pub struct Defer<'a, I, P>
where
    I: AsRef<str>,
    P: ?Sized,
{
    pub id: &'a I,
    pub payload: &'a P,
    pub delay: Option<Duration>,
}
