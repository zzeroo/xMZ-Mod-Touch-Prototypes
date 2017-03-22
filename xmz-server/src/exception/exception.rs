use action::Action;


pub trait HasException {
    fn check_exceptions(&self);
}

#[derive(Debug)]
pub struct Exception {
    actions: Vec<Action>,
}

impl Exception {
    pub fn new() -> Self {
        Exception {
            actions: vec![],
        }
    }
}
