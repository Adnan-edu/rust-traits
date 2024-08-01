pub trait Container<T> {
    pub fn get(&mut self) -> Option<T>;
    pub fn put(&mut self, item: T);
    pub fn is_empty(&self) -> bool;
}
