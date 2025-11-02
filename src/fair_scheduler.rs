use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Task {
    task_id: String,
    priority: f32,
}

impl Task {
    pub fn new(task_id: String, user_id: String, priority: f32) -> Task {
        Task {
            task_id: task_id,
            priority: priority,
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Task) -> bool {
        if self.task_id == other.task_id && self.priority == self.priority {
            return true;
        }
        return false;
    }
}

// This is a stateful scheduler,
// It internally maintains state of the global list of users

pub struct FairScheduler {
    system_capacity: usize,
    user_tasks_map: HashMap<String, VecDeque<Task>>,
}

impl FairScheduler {
    pub fn new(system_capacity: usize) -> FairScheduler {
        FairScheduler {
            system_capacity: system_capacity,
            user_tasks_map: HashMap::new(),
        }
    }

    pub fn run_cycle(&mut self, current_system_usage: usize) -> Vec<Task> {
        // While n-c != 0
        // Fetch top n-c users from the user_tasks_map
        //
        // Go through the user_tasks_map, and pick one task for each user with the highest cycles
        // waiting for.
        // Remove this task.

        let mut tasks_to_send_count = self.system_capacity - current_system_usage;
        let mut final_task_list: Vec<Task> = Vec::new();

        let mut last_iteration_useful: bool = true;

        while tasks_to_send_count > 0 && last_iteration_useful {
            last_iteration_useful = false;
            for (_, task_list) in self.user_tasks_map.iter_mut() {
                if task_list.len() == 0 {
                    continue;
                }

                final_task_list.push(task_list.pop_front().unwrap());
                last_iteration_useful = true;

                tasks_to_send_count -= 1;
                if tasks_to_send_count == 0 {
                    break;
                }
            }
        }

        return final_task_list;
    }

    pub fn add_task(&mut self, user_id: &String, task: Task) {
        // A new task is always pushed to the end of the list.
        // This means that tasks that have been waiting the longest will always be at the
        // front.
        if self.user_tasks_map.contains_key(user_id) {
            let mut task_queue: &mut VecDeque<Task> = self.user_tasks_map.get_mut(user_id).unwrap();
            task_queue.push_back(task);
        }
        else {
            self.user_tasks_map.insert(user_id.clone(), VecDeque::from([task]));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut fs: FairScheduler = FairScheduler::new(100);
        let task = Task::new("1".to_string(), "user1".to_string(), 0.3);
        fs.add_task(&"user1".to_string(), task);

        let test_task = Task::new("1".to_string(), "user1".to_string(), 0.3);
        let final_task_list = fs.run_cycle(0);
        for t in final_task_list {
            println!("Hmmm");
            assert_eq!(test_task, t);
        }
    }
}
