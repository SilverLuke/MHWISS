pub struct Skill {
    pub id: u16,
    pub name: String,
    pub description: String,
    pub dependency: Option<u8>,
    pub max_level: u8,
}

impl Skill{
    pub fn new(id: u16, name: String, description: String, max_level: u8) -> Self {
        Skill { id, name, description, max_level, dependency: None }
    }
}