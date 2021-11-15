use super::base::Base;
use super::traits::{get::Get, delete::Delete, edit::Edit};

pub struct Done {
    pub super_struct: Base // I don't like the way the author does this.
    // Rust doesn't have struct inheritance, so don't force it.  Use traits.
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        Done {
            super_struct: Base::new(input_title, "done")
        }
    }
}

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}