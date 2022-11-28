pub(crate) trait Callable {
    type Output;
    fn call(self) -> Self::Output;
}
