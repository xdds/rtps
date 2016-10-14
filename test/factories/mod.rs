pub trait Create {
    fn create() -> Self where Self: Sized { unreachable!() }
    fn create_variant(_ : &str) -> Self where Self: Sized { unreachable!() }
}

mod locator_factory;
pub use self::locator_factory::*;

mod stateless_writer_factory;
pub use self::stateless_writer_factory::*;

mod submessage_factory;
pub use self::submessage_factory::*;