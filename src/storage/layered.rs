
#[derive(Debug, Clone)]
pub struct LayeredStorage<Inner: Storage, Next: Storage> {
    inner: Arc<Inner>,
    next: Arc<Next>,
}
