use std::collections::HashMap;

use fair_scheduler::FairScheduler;
use fair_scheduler::Task;
use plotly::common::Line;
use plotly::common::Marker;
use plotly::layout::BarMode;
use plotly::Bar;
use plotly::Layout;
use plotly::Plot;
use plotly::Scatter;
use nanorand::{Rng, WyRand};

#[test]
fn test_users_less_than_n() {
    let n: usize = 50;
    let mut fs: FairScheduler = FairScheduler::new(n);
    let users = vec!["user1", "user2", "user3", "user4", "user5"];
    for i in 0..250 {
        let user_id = users[0].to_string();
        let task_id = i.to_string();
        fs.add_task(user_id, task_id, 1.0);
    }
    for i in 0..500 {
        let user_id = users[1].to_string();
        let task_id = i.to_string();
        fs.add_task(user_id, task_id, 1.0);
    }
    for i in 0..100 {
        let user_id = users[2].to_string();
        let task_id = i.to_string();
        fs.add_task(user_id, task_id, 1.0);
    }
    for i in 0..120 {
        let user_id = users[3].to_string();
        let task_id = i.to_string();
        fs.add_task(user_id, task_id, 1.0);
    }
    for i in 0..1200 {
        let user_id = users[4].to_string();
        let task_id = i.to_string();
        fs.add_task(user_id, task_id, 1.0);
    }
    // render_graph(&mut fs, n);
}

#[test]
fn test_users_more_than_n() {
    let mut rng = WyRand::new();
    let n: usize = rng.generate_range(5..50);
    let number_of_users: usize = rng.generate_range(5..10);
    let mut fs: FairScheduler = FairScheduler::new(n);
    let mut users: Vec<String> = Vec::new();
    let mut total_tasks: usize = 0;

    for i in 0..number_of_users {
        users.push(format!("user{}", i).to_string());
    }
    for user in users.iter() {
        let user_id = user;
        let task_count = rng.generate_range(5..10);
        for i in 0..task_count {
            let task_id = format!("dummy_task_id{}", i).to_string();
            fs.add_task(user_id.clone(), task_id.to_string(), 1.0);
            total_tasks += 1;
        }
    }
    println!("_Start state_");
    println!("Number of users: {}", number_of_users);
    println!("System capacity: {}", n);
    println!("Total tasks to process: {}", total_tasks);

    let total_cycles_ran = render_graph(&mut fs, n);
    println!("Total cycles ran: {}", total_cycles_ran);
}

fn render_graph(fs: &mut FairScheduler, system_capacity: usize) -> usize {
    let mut plot = Plot::new();
    let mut rng = WyRand::new();

    let mut output_task_list: Vec<Task> = vec![];
    let mut cycle_count = 0;
    let mut user_id_to_number_of_tasks_done: HashMap<String, Vec<usize>> = HashMap::new();

    while fs.get_current_task_count() != 0  || cycle_count == 0 {
        let tasks_currently_in_system = rng.generate_range(0..=system_capacity);
        output_task_list = fs.run_cycle(tasks_currently_in_system);
        for task in output_task_list.iter() {
            let user_id: &String = task.get_user_id();
            let tasks_done_for_user: &mut Vec<usize>;

            match user_id_to_number_of_tasks_done.get_mut(user_id) {
                Some(t) => tasks_done_for_user = t,
                None => {
                    user_id_to_number_of_tasks_done.insert(user_id.clone(), vec![]);
                    tasks_done_for_user = user_id_to_number_of_tasks_done.get_mut(user_id).unwrap();
                }
            }

            if tasks_done_for_user.len() < cycle_count + 1 {
                for _ in tasks_done_for_user.len()..=cycle_count {
                    tasks_done_for_user.push(0);
                }
            }

            tasks_done_for_user[cycle_count] = tasks_done_for_user[cycle_count] + 1;

        }

        cycle_count += 1;
    }

    for (user_id, task_vector) in user_id_to_number_of_tasks_done {
        let trace = Bar::new((1..=cycle_count).collect(), task_vector)
            .name(format!("{}", user_id))
            .opacity(0.5);
        plot.add_trace(trace);
    }

    let layout = Layout::new().bar_mode(BarMode::Stack);
    plot.set_layout(layout);

    plot.show();

    return cycle_count;
}
