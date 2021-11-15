use super::base::Base;

pub struct Pending {
    pub super_struct: Base
}

impl Pending {
    pub fn new(input_title: &str) -> Pending {
        Pending {
            super_struct: Base::new(input_title, "pending")
        }
    }
}