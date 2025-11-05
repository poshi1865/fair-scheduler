use std::collections::{HashMap, HashSet, VecDeque};

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
        let mut tasks_to_send_count = self.system_capacity - current_system_usage;
        let mut final_task_list: Vec<Task> = Vec::new();

        let mut last_iteration_useful: bool = true;

        while tasks_to_send_count > 0 && last_iteration_useful {
            // Find out n-c users that have been waiting the longest. Then iterate through them and
            // get tasks
            let users_waiting_longest: Vec<&User> = Vec::new();
            for user in self.users.iter() {
            }

            last_iteration_useful = false;
            for (_, user) in self.users.iter_mut() {
                if user.task_list.len() == 0 {
                    continue;
                }

                final_task_list.push(user.task_list.pop_front().unwrap());
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

        assert_eq!(fs.users.get("user0").unwrap().task_list.len(), 49);
        assert_eq!(fs.users.get("user1").unwrap().task_list.len(), 0);
        assert_eq!(fs.users.get("user2").unwrap().task_list.len(), 1);
        assert_eq!(fs.users.get("user15").unwrap().task_list.len(), 0);

        fs.run_cycle(0);
        assert_eq!(fs.users.get("user0").unwrap().task_list.len(), 25);
        assert_eq!(fs.users.get("user2").unwrap().task_list.len(), 0);
        assert_eq!(fs.get_current_task_count(), 25);
    }
}
