use crate::{BlockStore, PublicDirectory};
use chrono::Utc;
use std::rc::Rc;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct MutablePublicDirectory<'b, B: BlockStore> {
    pub store: &'b mut B,
    pub root: Rc<PublicDirectory>,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<'b, B: BlockStore> MutablePublicDirectory<'b, B> {
    pub fn new(store: &'b mut B) -> MutablePublicDirectory<'b, B> {
        Self {
            store,
            root: Rc::new(PublicDirectory::new(Utc::now())),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::utils::test_setup;

    use super::*;

    #[test]
    fn can_create_an_empty_directory() {
        let store = test_setup::init!(mut store);
        let dir = MutablePublicDirectory::new(store);

        println!("Directory: {:#?}", dir);
    }
}
