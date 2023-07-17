use serde::{Deserialize, Serialize};

use crate::profile::Status;

pub trait Task {
    fn is_match(user_status: Status) -> bool;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskList {
    pub task_id: String,
    pub event_chain: Vec<EventNode>,
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EventNode {
    pub order: u8,
    pub event: String,
    pub condition: Condition,
    pub result: String,
    pub participant: Vec<String>,
    pub script: Vec<Script>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Condition {
    pub location: u64,
    pub tool: Vec<u32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Script {
    pub user: String,
    pub content: String,
}

impl Task for TaskList {
    fn is_match(user_status: Status) -> bool {
        todo!()
    }
}

impl TaskList {
    pub fn load(json_conf: String) -> Vec<TaskList> {
        let task_list: Vec<TaskList> =
            serde_json::from_str(json_conf.as_str()).expect("load task json conf error");
        task_list
    }
}

mod test {
    use super::TaskList;

    #[test]
    fn test_task_load() {
        let data = r#"
        [{"task_id":"LaNi", "label":"拉妮A线",
        "event_chain": [
        {"order":1, "event": "enter_church", "condition": {"location": 12, "tool": [3,2]}, "participant":["lani"],  "result": "open", "script": [{"user":"lani","content": "welcome church"}] }
        ]
        }]
        "#;
        let task_list: Vec<TaskList> = TaskList::load(data.into());
        assert_eq!(task_list.len(), 1);
        let task = task_list.get(0).unwrap();
        assert_eq!(task.event_chain.len(), 1);
        let node = task.event_chain.get(0).unwrap();
        assert_eq!(node.result, "open");
    }
}
