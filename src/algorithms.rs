pub trait Step {
    fn step(&mut self, arr: &mut Vec<f64>, comparisons: &mut u64, accesses: &mut u64) -> Option<bool>;
}

pub struct BubbleSort {
    pub current_index: usize,
    pub current_iter_made_switch: bool,
    pub sorted: bool
}

impl Default for BubbleSort {
    fn default() -> Self {
        Self {
            current_index: 0,
            current_iter_made_switch: false,
            sorted: false
        }
    }
}

impl Step for BubbleSort {
    fn step(&mut self, arr: &mut Vec<f64>, comparisons: &mut u64, accesses: &mut u64) -> Option<bool> {
        *comparisons += 1;
        *accesses += 2; // 2 for the comparison
        if arr[self.current_index] > arr[self.current_index + 1] {
            self.current_iter_made_switch = true;
            let temp = arr[self.current_index];
            *accesses += 1;
            arr[self.current_index] = arr[self.current_index + 1];
            *accesses += 2;
            arr[self.current_index + 1] = temp;
            *accesses += 1;
        }
        self.current_index += 1;
        if self.current_index == arr.len() - 1 {
            self.current_index = 0;
            self.sorted = !self.current_iter_made_switch;
            self.current_iter_made_switch = false;
            return Some(self.sorted); // Returns Some if this is the last iteration/step, and a bool representing whether or not it has finished sorting.
        }
        return None; // Returns None if this is not the last iteration/step
    }
}