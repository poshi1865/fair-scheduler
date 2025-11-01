use std::collections::{HashMap, HashSet, VecDeque};

pub struct Task {
    task_id: String,
    priority: f32,
    user_id: String,
    cycles_waiting_for: usize,
}

impl Task {
    pub fn new(task_id: String, user_id: String, priority: f32) -> Task {
        Task {
            task_id: task_id,
            user_id: user_id,
            priority: priority,
            cycles_waiting_for: 0
        }
    }
}

// This is a stateful scheduler,
// It internally maintains state of the global list of users

pub struct FairScheduler {
    system_capacity: usize,
    task_queue: VecDeque<Task>,
    user_to_cycles_since_last_served_map: HashMap<String, usize>
}

impl FairScheduler {
    pub fn new(system_capacity: usize) -> FairScheduler {
        FairScheduler {
            system_capacity: system_capacity,
            task_queue: VecDeque::new(),
            user_to_cycles_since_last_served_map: HashMap::new()
        }
    }

    pub fn run_cycle(&mut self, current_system_usage: usize) -> Vec<Task> {
        // While n-c != 0
        // Fetch top n-c users from the user_to_cycles_since_last_served.
        //
        // Go through the task queue, and pick one task for each user in the above fetch
        // Remove this task from the task_queue

        // Update the users_served list with new users from the queue, if any
        for task in &self.task_queue {
            if !self.user_to_cycles_since_last_served_map.contains_key(&task.user_id) {
                self.user_to_cycles_since_last_served_map.insert(task.user_id.clone(), 0);
            }
        }

        let tasks_to_send_count = self.system_capacity - current_system_usage;
        
        let tasks_to_send: Vec<Task> = Vec::new();

        let mut sorted_users = convert_map_into_vector_and_sort(&self.user_to_cycles_since_last_served_map);

        while tasks_to_send_count != 0 {
            for user in sorted_users.iter_mut() {
            }
        }

        return tasks_to_send;

    }

    pub fn add_task(&mut self, task: Task) {
        self.task_queue.push_back(task);
    }
}

fn convert_map_into_vector_and_sort(map: &HashMap<String, usize>) -> Vec<(String, usize)> {
    // Convert HashMap into a vector of (key, value) pairs
    let mut vec: Vec<(String, usize)> = map.iter().map(|(k, v)| (k.clone(), *v)).collect();
    // Sort by value (descending)
    vec.sort_by_key(|k| std::cmp::Reverse(k.1));
    vec
}
