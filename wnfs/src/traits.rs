use std::rc::Rc;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// Implements getting a unique identifier for a node.
pub trait Id {
    /// Gets an identifier for the node.
    fn get_id(&self) -> String;
}

/// This trait exists to allow types that can be `Rc::make_mut`'d to do additional things like
/// setting up revision correctly before the `Rc::make_mut` call.
pub(crate) trait PrepareMut {
    fn prepare_mut<'a>(self: &'a mut Rc<Self>) -> &'a mut Self;
}
