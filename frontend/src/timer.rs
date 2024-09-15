/// A timer with an id, name, duration, and start time.
/// 
/// The `id` is used to identify the timer in the list of timers.
/// The `name` is the name of the timer.
/// The `duration` is the duration of the timer in seconds.
/// The `is_running` is a boolean that indicates if the timer is running.
/// The `start_time` is the time when the timer started in seconds.
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