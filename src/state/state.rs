use super::notification_state::NotificationState;

pub struct AppState {
    pub shuffle: bool,
    pub notification: NotificationState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            shuffle: false,
            notification: NotificationState::default(),
        }
    }
}
