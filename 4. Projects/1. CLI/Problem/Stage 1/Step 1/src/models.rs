use serde_derive::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Default, Debug)]
pub enum Status {
    #[default]
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Epic {
    pub name: String,
    pub descrption: String,
    pub status : Status,
    pub stories: Vec<i32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        let mut epic = Epic::default();
        epic.set_name(name);
        epic.set_description(description);
        epic
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_description(&mut self, description: String) {
        self.descrption = description;
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Story {    
    pub name: String,
    pub descrption: String,
    pub status : Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Story::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DBState {
    pub item_counter: u128,
    pub epics: Vec<Epic>,
    pub stories: Vec<Story>,
}

impl DBState {
    fn new() -> DBState {
        DBState::default()
    }        
}