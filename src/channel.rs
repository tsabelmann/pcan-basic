//!
//!

///
pub trait UninitializedChannel {
    ///
    fn uninitialized_channel(&self) -> u16;
}

///
pub trait InitializedChannel {
    ///
    fn initialized_channel(&self) -> u16;
}
