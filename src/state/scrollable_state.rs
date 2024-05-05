use ratatui::widgets::{ListState, TableState};

pub trait ScrollableStateInternal {
    fn selected(&self) -> Option<usize>;
    fn selected_mut(&mut self) -> &mut Option<usize>;
    fn default_with_selected(idx: Option<usize>) -> Self;
}

impl ScrollableStateInternal for ListState {
    fn selected(&self) -> Option<usize> {
        self.selected()
    }

    fn selected_mut(&mut self) -> &mut Option<usize> {
        self.selected_mut()
    }

    fn default_with_selected(idx: Option<usize>) -> Self {
        ListState::default().with_selected(idx)
    }
}

impl ScrollableStateInternal for TableState {
    fn selected(&self) -> Option<usize> {
        self.selected()
    }

    fn selected_mut(&mut self) -> &mut Option<usize> {
        self.selected_mut()
    }

    fn default_with_selected(idx: Option<usize>) -> Self {
        TableState::default().with_selected(idx)
    }
}

pub struct ScrollableState<T, S> {
    pub state: S,
    pub options: Vec<T>
} 

impl<T: Clone, S: ScrollableStateInternal> ScrollableState<T, S> {
    pub fn get_current_selection_idx(&self) -> Option<usize> {
        match self.state.selected() {
            Some(idx) => Some(idx),
            None => None
        }
    }

    pub fn get_current_selection(&self) -> Option<T> {
        let curr_idx = match self.state.selected() {
            Some(idx) => idx,
            None => return None
        };

        match self.options.get(curr_idx) {
            Some(s) => Some(s.clone()),
            None => None
        }
    }

    pub fn scroll_up_single(&mut self) {
        self.scroll(-1);
    }

    pub fn scroll_down_single(&mut self) {
        self.scroll(1);
    }

    pub fn scroll(&mut self, n: i32) {
        let curr_idx = match self.state.selected_mut() {
            Some(x) => x,
            None => return,
        };
        *curr_idx = (curr_idx.clone() as i32 + n).clamp(0, self.options.len() as i32 - 1) as usize;
    }

    pub fn set_scroll(&mut self, n: usize) {
        let curr_idx = match self.state.selected_mut() {
            Some(x) => x,
            None => return,
        };
        *curr_idx = n;
    }

    pub fn set_scroll_last(&mut self) {
        let last_idx = self.options.len() - 1;
        self.set_scroll(last_idx);
    }
}

impl<T, S> Default for ScrollableState<T, S> 
where S: Default + ScrollableStateInternal 
{
    fn default() -> Self {
        ScrollableState {
            state: S::default_with_selected(Some(0)),
            options: Vec::new()
        }
    }
}
