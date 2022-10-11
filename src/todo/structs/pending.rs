use super::base::Base;
use super::traits::create::Create;
pub struct Pending{
    pub super_struct: Base
}

impl Pending{
    pub fn new(input_title: &str)-> Pending{
        let base: Base = Base::new(input_title, "pending");
        return Pending {
            super_struct: base
        }
    }
}

impl Create for Pending {}