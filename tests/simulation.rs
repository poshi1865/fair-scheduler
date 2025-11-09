use std::collections::HashMap;

use fair_scheduler::FairScheduler;
use fair_scheduler::Task;
use plotly::common::Marker;
use plotly::Plot;
use plotly::Scatter;

#[test]
fn test_all_users_equal() {
    let n: usize = 25;

    let mut fs: FairScheduler = FairScheduler::new(n);

    let mut users = vec!["user1", "user2", "user3", "user4", "user5"];

    for i in 0..25 {
        let user_id = users[0].to_string();
        let task_id = i.to_string();
        fs.add_task(user_id, task_id, 1.0);
    }
    for i in 25..50 {
        let user_id = users[1].to_string();
        let task_id = i.to_string();
        fs.add_task(user_id, task_id, 1.0);
    }
    for i in 50..75 {
        let user_id = users[2].to_string();
        let task_id = i.to_string();
        fs.add_task(user_id, task_id, 1.0);
    }
    for i in 75..100 {
        let user_id = users[3].to_string();
        let task_id = i.to_string();
        fs.add_task(user_id, task_id, 1.0);
    }
    for i in 100..200 {
        let user_id = users[4].to_string();
        let task_id = i.to_string();
        fs.add_task(user_id, task_id, 1.0);
    }

    let mut output_task_list: Vec<Task> = fs.run_cycle(0);
    let mut plot = Plot::new();

    let mut cycle_count = 1;
    let mut user_id_to_number_of_tasks_done: HashMap<String, Vec<usize>> = HashMap::new();
    while output_task_list.len() != 0 {
        for task in output_task_list {
            let user_id: &String = task.get_user_id();
            
            let mut task_vector: &mut Vec<usize>;
            match user_id_to_number_of_tasks_done.get_mut(user_id) {
                Some(t) => task_vector = t,
                None => {
                    user_id_to_number_of_tasks_done.insert(user_id.clone(), vec![]);
                    task_vector = user_id_to_number_of_tasks_done.get_mut(user_id).unwrap();
                }
            }

            if task_vector.len() < cycle_count {
                task_vector.push(0);
            }

            task_vector.push(task_vector[cycle_count - 1] + 1);

        }

        cycle_count += 1;

        output_task_list = fs.run_cycle(0);
    }

    for (user_id, task_vector) in user_id_to_number_of_tasks_done {
        println!("{} :: {:?}", user_id, task_vector);
        let trace = Scatter::new((1..=cycle_count).collect(), task_vector)
            .mode(plotly::common::Mode::Markers)
            .name(user_id.to_string())
            .marker(Marker::new());
        plot.add_trace(trace);
    }

    plot.show();

}

// #[test]
// fn test_one_user_heavy() {
//     let n: usize = 25;
//     let mut fs: FairScheduler = FairScheduler::new(n);
//
//     for i in 1..n {
//         let user_id = format!("user{}", i);
//         let task = Task::new(i.to_string(), 0.5);
//         fs.add_task(user_id, task);
//     }
//
//     for i in 0..50 {
//         let user_id = format!("user0");
//         let task = Task::new(i.to_string(), 0.5);
//         fs.add_task(user_id, task);
//     }
//
//     let temp_task = Task::new("10000".to_string(), 0.5);
//     fs.add_task("user2".to_string(), temp_task);
//
//     fs.run_cycle(0);
//
//     assert_eq!(fs.get_task_list_for_user("user0").len(), 49);
//     assert_eq!(fs.get_task_list_for_user("user1").len(), 0);
//     assert_eq!(fs.get_task_list_for_user("user2").len(), 1);
//     assert_eq!(fs.get_task_list_for_user("user15").len(), 0);
//
//     fs.run_cycle(0);
//     assert_eq!(fs.get_task_list_for_user("user0").len(), 25);
//     assert_eq!(fs.get_task_list_for_user("user2").len(), 0);
//     assert_eq!(fs.get_current_task_count(), 25);
// }
