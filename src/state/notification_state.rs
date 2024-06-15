use stopwatch::Stopwatch;

pub struct NotificationState {
    pub message: String,
    pub stopwatch: Stopwatch,
    pub time_limit_s: Option<u64>,
}

impl Default for NotificationState {
    fn default() -> Self {
        Self {
            message: String::new(),
            stopwatch: Stopwatch::new(),
            time_limit_s: None
        }
    }
}

impl NotificationState {
    // sets message in bottom panel, pass time_limit_s = None to display indefinitely
    pub fn set_message(&mut self, msg: String, time_limit_s: Option<u64>) {
        self.clear_message();
        self.message = msg;
        self.time_limit_s = time_limit_s;
        if self.time_limit_s.is_some() {
            self.stopwatch.start();
        }
    }

    // Clears displayed message
    pub fn clear_message(&mut self) {
        self.message = String::new();
        self.time_limit_s = None;
        if self.stopwatch.is_running() {
            self.stopwatch.stop();
        }

        self.stopwatch.reset();
    }

    // clears message if time since message was displayed is greater than time limit
    pub fn update_message(&mut self) {
        if let Some(l) = self.time_limit_s {
            if self.stopwatch.elapsed().as_secs() > l {
                self.clear_message();
            }
        }
    }
}
