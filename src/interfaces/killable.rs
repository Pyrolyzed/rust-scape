pub trait Killable {
    fn is_alive(&self) -> bool;
    fn die(&mut self);
}