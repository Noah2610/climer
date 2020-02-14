#[derive(Clone)]
pub enum TimerState {
    Stopped,
    Running,
    Paused,
    Finished,
}

impl TimerState {
    pub fn is_stopped(&self) -> bool {
        if let TimerState::Stopped = self {
            true
        } else {
            false
        }
    }

    pub fn is_running(&self) -> bool {
        if let TimerState::Running = self {
            true
        } else {
            false
        }
    }

    pub fn is_paused(&self) -> bool {
        if let TimerState::Paused = self {
            true
        } else {
            false
        }
    }

    pub fn is_finished(&self) -> bool {
        if let TimerState::Finished = self {
            true
        } else {
            false
        }
    }
}
