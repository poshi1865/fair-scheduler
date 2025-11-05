use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct Task {
    task_id: String,
    priority: f32,
}

impl Task {
    pub fn new(task_id: String, priority: f32) -> Task {
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
    current_task_count: usize
}

impl FairScheduler {
    pub fn new(system_capacity: usize) -> FairScheduler {
        FairScheduler {
            system_capacity: system_capacity,
            user_tasks_map: HashMap::new(),
            current_task_count: 0
        }
    }

    pub fn run_cycle(&mut self, current_system_usage: usize) -> Vec<Task> {
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
                self.current_task_count -= 1;
                last_iteration_useful = true;

                tasks_to_send_count -= 1;
                if tasks_to_send_count == 0 {
                    break;
                }
            }
        }

        return final_task_list;
    }

    pub fn add_task(&mut self, user_id: String, task: Task) {
        // A new task is always pushed to the end of the list.
        // This means that tasks that have been waiting the longest will always be at the
        // front.
        if self.user_tasks_map.contains_key(&user_id) {
            let task_queue: &mut VecDeque<Task> = self.user_tasks_map.get_mut(&user_id).unwrap();
            task_queue.push_back(task);
        }
        else {
            self.user_tasks_map.insert(user_id.clone(), VecDeque::from([task]));
        }
        self.current_task_count += 1;
    }

    pub fn get_current_task_count(&self) -> usize {
        return self.current_task_count;
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::fmt::Display;
    use std::hash::Hash;

    use super::*;

    fn print_array<T: Debug>(v: &VecDeque<T>) {
        for i in v.iter() {
            print!("{:?}, ", i);
        }
        print!("\n");
    }

    fn print_map<K: Display+Eq+Hash, V: Debug>(map: &HashMap<K, VecDeque<V>>) {
        for i in map.keys() {
            print!("{}: ", i);
            for j in map.get(i).iter() {
                print_array(j);
            }
        }
    }

    #[test]
    fn test_add_task() {
        let mut fs: FairScheduler = FairScheduler::new(100);
        let task = Task::new("1".to_string(), 0.3);
        fs.add_task("user1".to_string(), task);

        let test_task = Task::new("1".to_string(), 0.3);
        let final_task_list = fs.run_cycle(0);
        for t in final_task_list {
            assert_eq!(test_task, t);
        }
    }

    #[test]
    fn test_all_users_equal() {
        let N: usize = 25;

        let mut fs: FairScheduler = FairScheduler::new(N);

        for i in 0..N {
            let user_id = format!("user{}", i);
            let task = Task::new(i.to_string(), 0.5);
            fs.add_task(user_id, task);
        }

        assert_eq!(fs.get_current_task_count(), N);

        let mut output_task_list = fs.run_cycle(10);

        assert_eq!(fs.get_current_task_count(), 10);
        assert_eq!(output_task_list.len(), 15);

        fs.run_cycle(5);

    }

    #[test]
    fn test_one_user_heavy() {
        let N: usize = 25;
        let mut fs: FairScheduler = FairScheduler::new(N);

        for i in 1..N {
            let user_id = format!("user{}", i);
            let task = Task::new(i.to_string(), 0.5);
            fs.add_task(user_id, task);
        }

        for i in 0..50 {
            let user_id = format!("user0");
            let task = Task::new(i.to_string(), 0.5);
            fs.add_task(user_id, task);
        }

        let temp_task = Task::new("10000".to_string(), 0.5);
        fs.add_task("user2".to_string(), temp_task);

        fs.run_cycle(0);

        assert_eq!(fs.user_tasks_map.get("user0").unwrap().len(), 49);
        assert_eq!(fs.user_tasks_map.get("user1").unwrap().len(), 0);
        assert_eq!(fs.user_tasks_map.get("user2").unwrap().len(), 1);
        assert_eq!(fs.user_tasks_map.get("user15").unwrap().len(), 0);

        fs.run_cycle(0);
        assert_eq!(fs.user_tasks_map.get("user0").unwrap().len(), 25);
        assert_eq!(fs.user_tasks_map.get("user2").unwrap().len(), 0);
        assert_eq!(fs.get_current_task_count(), 25);
    }
}
