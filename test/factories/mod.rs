pub trait Create {
    fn create() -> Self;
}

mod stateless_writer_factory;
pub use self::stateless_writer_factory::*;