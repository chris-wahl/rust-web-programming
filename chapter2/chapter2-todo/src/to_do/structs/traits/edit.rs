pub trait Edit {
    fn set_to_done(&mut self, title: &str) {
        println!("{} is being set to done", title);
    }
    fn set_to_pending(&mut self, title: &str) {
        println!["{} is being set to pending", title]
    }
}