use crate::prelude::*;

pub const TURN_TIME: u32 = 1000;
pub const WAIT_TIME: u32 = 1000;
pub const ATTACK_TIME: u32 = 1000;

#[derive(Debug, Reflect, FromReflect, Clone, Copy)]
pub enum ActionType {
    Wait,
    Attack(Position),
    Movement(Position),
    MovementDelta(IVec2),
}

impl ActionType {
    pub const fn get_base_time_to_perform(&self) -> u32 {
        match self {
            Self::Wait => WAIT_TIME,
            Self::Attack(_) => ATTACK_TIME,
            Self::Movement(_) => TURN_TIME,
            Self::MovementDelta(_) => TURN_TIME,
        }
    }
}
