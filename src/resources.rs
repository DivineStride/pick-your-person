use bevy::prelude::*;

#[derive(Resource)]
pub struct CountdownTimer {
    pub timer: Timer,
}

impl Default for CountdownTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(3.0, TimerMode::Once)
        }
    }
}

#[derive(Resource)]
pub struct WaitingTimer {
    pub timer: Timer,
}

impl Default for WaitingTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds( 2.0, TimerMode::Once),
        }
    }
}

#[derive(Resource)]
pub struct WinnerDisplayTimer(pub Timer);
