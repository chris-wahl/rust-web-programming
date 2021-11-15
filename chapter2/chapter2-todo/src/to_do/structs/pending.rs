use super::base::Base;
use super::traits::{create::Create, delete::Delete, edit::Edit, get::Get};

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Pending {
        Pending {
            super_struct: Base::new(input_title, "pending")
        }
    }
}


impl Get for Pending {}

impl Delete for Pending {}

impl Edit for Pending {}

impl Create for Pending {}