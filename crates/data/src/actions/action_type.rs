use crate::prelude::*;

pub const TURN_TIME: u32 = 1000;
pub const WAIT_TIME: u32 = 1000;

#[derive(Debug, Reflect, FromReflect)]
pub enum ActionType {
    Wait,
    Movement(IVec2),
}

impl ActionType {
    pub fn get_base_time_to_perform(&self) -> u32 {
        match self {
            Self::Wait => WAIT_TIME,
            Self::Movement(_) => TURN_TIME,
        }
    }
}