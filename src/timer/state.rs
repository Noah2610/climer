#[derive(Clone, PartialEq, Eq)]
pub enum TimerState {
    Stopped,
    Running,
    Paused,
    Finished,
}

impl TimerState {
    pub fn is_stopped(&self) -> bool {
        *self == TimerState::Stopped
    }

    pub fn is_running(&self) -> bool {
        *self == TimerState::Running
    }

    pub fn is_paused(&self) -> bool {
        *self == TimerState::Paused
    }

    pub fn is_finished(&self) -> bool {
        *self == TimerState::Finished
    }
}
