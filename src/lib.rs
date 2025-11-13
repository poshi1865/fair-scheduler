use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Task {
    task_id: String,
    priority: f32,
    user_id: String
}

impl Task {
    pub fn new(task_id: String, user_id: String, priority: f32) -> Task {
        Task {
            task_id: task_id,
            priority: priority,
            user_id: user_id
        }
    }

    pub fn get_user_id(&self) -> &String {
        return &self.user_id;
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

struct User {
    id: String,
    cycles_waiting: usize,
    task_list: VecDeque<Task>,
}
impl User {
    pub fn new(id: String, cycles_waiting: usize, task_list: VecDeque<Task>) -> User {
        User {
            id: id,
            cycles_waiting: cycles_waiting,
            task_list: task_list
        }
    }
}
// The scheduler internally maintains state of the global list of users

pub struct FairScheduler {
    system_capacity: usize,
    users: HashMap<String, User>,
    current_task_count: usize
}

impl FairScheduler {
    pub fn new(system_capacity: usize) -> FairScheduler {
        FairScheduler {
            system_capacity: system_capacity,
            users: HashMap::new(),
            current_task_count: 0
        }
    }

    pub fn run_cycle(&mut self, current_system_usage: usize) -> Vec<Task> {
        let mut max_tasks_can_send = self.system_capacity - current_system_usage;
        if max_tasks_can_send == 0 {
            return vec![];
        }
        let mut final_task_list: Vec<Task> = Vec::new();

        // Find out n-c users that have been waiting the longest. Then iterate through them and
        // get tasks
        let mut sorted_list: Vec<(&String, &mut User)> = Vec::new();
        for (id, user) in self.users.iter_mut() {
            sorted_list.push((id, user));
        }
        // sorted_list.sort_by(|a, b| b.1.cycles_waiting.cmp(&a.1.cycles_waiting));
        sorted_list.sort_by_key(|(_, user)| std::cmp::Reverse(user.cycles_waiting));

        let mut all_users_served = false;
        let mut last_iteration_useful = true;
        while max_tasks_can_send != 0 && last_iteration_useful {
            last_iteration_useful = false;
            for (_, user) in sorted_list.iter_mut() {
                if user.task_list.len() == 0 {
                    continue;
                }
                if max_tasks_can_send == 0 && !all_users_served {
                    user.cycles_waiting += 1;
                }
                else if max_tasks_can_send == 0 && all_users_served {
                    break;
                }
                else {
                    user.cycles_waiting = 0;
                    final_task_list.push(user.task_list.pop_front().unwrap());
                    self.current_task_count -= 1;
                    max_tasks_can_send -= 1;
                    last_iteration_useful = true;
                }
            }
            all_users_served = true;
        }

        return final_task_list;
    }

    pub fn add_task(&mut self, user_id: String, task_id: String, priority: f32) {
        // A new task is always pushed to the end of the list.
        // This means that tasks that have been waiting the longest will always be at the
        // front.
        let task = Task::new(task_id, user_id.clone(), priority);
        if self.users.contains_key(&user_id) {
            let task_queue: &mut VecDeque<Task> = &mut self.users.get_mut(&user_id).unwrap().task_list;
            task_queue.push_back(task);
        }
        else {
            self.users.insert(user_id.clone(), User::new(user_id.clone(), 0, VecDeque::from([task])));
        }
        self.current_task_count += 1;
    }

    pub fn get_current_task_count(&self) -> usize {
        return self.current_task_count;
    }

    pub fn get_task_list_for_user(&self, user_id: &str) -> &VecDeque<Task> {
        return &self.users.get(user_id).unwrap().task_list;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use plotly::{Plot, Scatter};

    #[test]
    fn test_add_task() {
        let mut fs: FairScheduler = FairScheduler::new(100);
        let user_id = "user1";
        let task_id = "task1";
        fs.add_task(user_id.to_string(), task_id.to_string(), 0.3);

        let test_task = Task::new(task_id.to_string(), user_id.to_string(), 0.3);

        let final_task_list = fs.run_cycle(0);
        assert_eq!(final_task_list[0], test_task);
    }

}
