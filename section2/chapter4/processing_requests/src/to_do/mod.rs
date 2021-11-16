use structs::{done::Done, pending::Pending};

pub(crate) mod structs;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done)
}

pub fn to_do_factory(item_type: &str, item_title: &str) -> Result<ItemTypes, &'static str> {
    // This a weird decision by the author.  Use the Enum!!  And Matching!!
    if item_type == "pending" {
        let pending_item = Pending::new(item_title);
        Ok(ItemTypes::Pending(pending_item))
    } else if item_type == "done" {
        let done_item = Done::new(item_title);
        Ok(ItemTypes::Done(done_item))
    } else {
        Err("Type not accepted")
    }
}