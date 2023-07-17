use crate::profile::Status;

pub trait Effect {
    fn can_use(user_status: Status) -> bool;
}

pub struct Skill {
    pub sid: u8,
    pub name: String,
    pub mp_cost: u64,
    pub hp_cost: u64,
    pub damage: u64,
}

impl Effect for Skill {
    fn can_use(user_status: Status) -> bool {
        todo!()
    }
}

pub struct Weapon {
    pub wid: u8,
    pub name: String,
    pub limit: u8, //限制装备条件
    pub damage: u64,
}
