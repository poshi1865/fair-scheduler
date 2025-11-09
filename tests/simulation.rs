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

#[test]
fn test1() {
    let n: usize = 25;
    let mut fs: FairScheduler = FairScheduler::new(n);
    let users = vec!["user1", "user2", "user3", "user4", "user5"];
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
    // render_graph(&mut fs);
}

#[test]
fn test2() {
    let n: usize = 25;
    let mut fs: FairScheduler = FairScheduler::new(n);
    let users = vec!["user1", "user2", "user3", "user4", "user5"];
    for i in 0..2 {
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
    render_graph(&mut fs);
}

fn render_graph(fs: &mut FairScheduler) {
    let mut output_task_list: Vec<Task> = vec![];
    let mut plot = Plot::new();

    let mut cycle_count = 0;
    let mut user_id_to_number_of_tasks_done: HashMap<String, Vec<usize>> = HashMap::new();
    while output_task_list.len() != 0  || cycle_count == 0 {
        output_task_list = fs.run_cycle(0);
        for task in output_task_list.iter() {
            let user_id: &String = task.get_user_id();

            let task_vector: &mut Vec<usize>;
            match user_id_to_number_of_tasks_done.get_mut(user_id) {
                Some(t) => task_vector = t,
                None => {
                    user_id_to_number_of_tasks_done.insert(user_id.clone(), vec![]);
                    task_vector = user_id_to_number_of_tasks_done.get_mut(user_id).unwrap();
                }
            }

            if task_vector.len() < cycle_count + 1 {
                task_vector.push(0);
            }

            task_vector[cycle_count] = task_vector[cycle_count] + 1;

        }

        cycle_count += 1;
    }

    for (user_id, task_vector) in user_id_to_number_of_tasks_done {
        println!("{} :: {:?}", user_id, task_vector);
        let trace = Bar::new((1..=cycle_count).collect(), task_vector)
            .name(format!("{}", user_id))
            .opacity(0.5);
        plot.add_trace(trace);
    }

    let layout = Layout::new().bar_mode(BarMode::Stack);
    plot.set_layout(layout);

    plot.show();
}
