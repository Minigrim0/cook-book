/// A timer with an id, name, duration, and start time.
#[derive(Debug, Clone)]
pub struct Timer {
    pub id: i32,
    pub name: String,
    pub duration: u32,
    pub is_running: bool,
    pub elapsed_time: u32,
}

impl PartialEq for Timer {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Timer {

}