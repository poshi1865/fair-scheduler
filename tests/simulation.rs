use std::collections::HashMap;

use fair_scheduler::FairScheduler;
use fair_scheduler::Task;
use plotly::layout::BarMode;
use plotly::Bar;
use plotly::Layout;
use plotly::Plot;
use nanorand::{Rng, WyRand};

#[test]
fn test_random_simul() {
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

    let total_cycles_ran = render_graph(&mut fs, n, 2);
    println!("Total cycles ran: {}", total_cycles_ran);
}

fn render_graph(
    fs: &mut FairScheduler,
    system_capacity: usize,
    mut number_of_times_to_add_more_tasks: usize,
) -> usize {
    let mut plot = Plot::new();
    let mut rng = WyRand::new();

    let mut output_task_list: Vec<Task> = vec![];
    let mut cycle_count = 0;
    let mut user_id_to_number_of_tasks_done: HashMap<String, Vec<usize>> = HashMap::new();

    while fs.get_current_task_count() != 0 {

        if number_of_times_to_add_more_tasks != 0 {
            let users = vec!["usera", "userb", "userc", "userd"];
            for user in users.into_iter() {
                let task_count = rng.generate_range(5..10);
                for i in 0..task_count {
                    let user_id = user.to_string();

                    let task_id = format!("dummy_task_id-{}", i).to_string();
                    fs.add_task(user_id, task_id.to_string(), 1.0);
                }
            }

            number_of_times_to_add_more_tasks -= 1;
        }

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
