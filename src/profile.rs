use crate::{
    task::EventNode,
    things::{Skill, Weapon},
};

pub struct Profile {
    pub name: String,
    pub level: u16,
    pub tasks: Vec<EventNode>,
    pub status: Status,
    pub skills: Vec<Skill>,
    pub weapons: Vec<Weapon>,
}

pub struct Status {
    pub hp: u64,
    pub mp: u64,
    pub ls: u8, //strength
    pub li: u8, //intelligence
    pub la: u8, //agile
}
