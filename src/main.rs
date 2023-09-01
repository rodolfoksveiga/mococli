use std::{cell::RefCell, error::Error, fs::remove_file, fs::File, io::Write, sync::Arc, vec};

use crate::{
    moco::{client::MocoClient, model::EditActivity},
    utils::{ask_question, mandatory_validator},
};

use chrono::{Duration, Local, Utc};
use utils::{promp_activity_select, promp_task_select, render_table};

use itertools::Itertools;

use crate::moco::model::{
    ControlActivityTimer, CreateActivity, DeleteActivity, GetActivity, GetProjectTasks,
};

mod cli;
mod config;
mod moco;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::init();
    let mut log_builder = env_logger::builder();
    log_builder.parse_default_env();
    if args.debug {
        log_builder.filter_level(log::LevelFilter::Trace);
    }
    log_builder.init();
    let config = Arc::new(RefCell::new(config::init()?));
    let moco_client = MocoClient::new(&config);

    match args.command {
        cli::Commands::Login {} => {
            let moco_company = ask_question("Enter the company name: ", &mandatory_validator)?;
            let api_key = ask_question("Enter your personal API key: ", &mandatory_validator)?;
            let admin_api_key = ask_question("Enter the admin API key: ", &mandatory_validator)?;

            config.borrow_mut().moco_company = Some(moco_company);
            config.borrow_mut().moco_api_key = Some(api_key);
            config.borrow_mut().moco_admin_api_key = Some(admin_api_key);

            let firstname = ask_question("Enter your firstname: ", &mandatory_validator)?;
            let lastname = ask_question("Enter your lastname: ", &mandatory_validator)?;

            let client_id = moco_client.get_user_id(firstname, lastname).await?;

            config.borrow_mut().moco_user_id = client_id;
            config.borrow_mut().write_config()?;
            println!("You're logged in")
        }
        cli::Commands::List { today, week, month } => {
            let (from, to) = utils::select_from_to_date(today, week || !today && !month, month);

            let activities = moco_client
                .get_activities(
                    from.format("%Y-%m-%d").to_string(),
                    to.format("%Y-%m-%d").to_string(),
                )
                .await?;

            let mut list: Vec<Vec<String>> = activities
                .iter()
                .map(|activity| {
                    vec![
                        activity.date.clone(),
                        activity.hours.to_string(),
                        activity.customer.name.clone(),
                        activity.project.name.clone(),
                        activity.task.name.clone(),
                        activity
                            .description
                            .as_ref()
                            .unwrap_or(&String::new())
                            .to_string(),
                    ]
                })
                .collect();
            list.insert(
                0,
                vec![
                    "Date".to_string(),
                    "Duration".to_string(),
                    "Customer".to_string(),
                    "Project".to_string(),
                    "Task".to_string(),
                    "Description".to_string(),
                ],
            );

            render_table(list)
        }
        cli::Commands::New {
            project,
            task,
            date,
            hours,
            description,
        } => {
            let now = Utc::now().format("%Y-%m-%d").to_string();

            let (project, task) = promp_task_select(&moco_client, project, task).await?;

            let date = if let Some(d) = date {
                d
            } else {
                print!("Date (YYYY-MM-DD) - Default 'today': ");
                std::io::stdout().flush()?;

                let date = utils::read_line()?;
                if date.is_empty() {
                    now
                } else {
                    date
                }
            };

            let hours = if let Some(h) = hours {
                h
            } else {
                ask_question("Duration (hours) - Default 'start timer': ", &|answer| {
                    answer.parse::<f64>().err().map(|e| format!("{}", e))
                })?
                .parse::<f64>()?
            };

            let description = if let Some(d) = description {
                d
            } else {
                print!("Description: ");
                std::io::stdout().flush()?;

                let description = utils::read_line()?;
                if description.is_empty() {
                    "".to_string()
                } else {
                    description
                }
            };

            moco_client
                .create_activity(&CreateActivity {
                    date,
                    project_id: project.id,
                    task_id: task.id,
                    hours: Some(hours),
                    description: description.clone(),
                    ..Default::default()
                })
                .await?;

            if hours == 0.0 {
                let mut file = File::create("/home/rodox/.config/mococli/moco_timer")?;
                file.write(
                    format!(
                        "{} {}",
                        Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
                    )
                    .as_bytes(),
                )?;
            }

            let (from, to) = utils::select_from_to_date(true, false, false);

            let activities = moco_client
                .get_activities(
                    from.format("%Y-%m-%d").to_string(),
                    to.format("%Y-%m-%d").to_string(),
                )
            .await?;

            let today: Vec<f64> =  activities.iter().map(|activity| {
                activity.hours
            }).collect();

            let today_hours: f64 = today.iter().sum();

            let report = moco_client.get_performance_report().await?;

            let operator = if report.annually.variation_until_today > 0.00 {
                "+"
            } else {
                ""
            };

            let mut file = File::create("/home/rodox/.config/mococli/moco_report")?;
            file.write(
                format!(
                    "{}{} {}",
                    operator,
                    report.annually.variation_until_today.to_string(),
                    today_hours.to_string()
                )
                .as_bytes(),
            )?;

            println!(
                "Activity\n    Project: {}\n    Task: {}\n    Duration: {} h\n    Description: {}",
                project.name,
                task.name,
                hours,
                description.clone()
            )
        }
        cli::Commands::Edit {
            activity,
            project,
            task,
            date,
            hours,
            description,
        } => {
            let activity = if let Some(a) = activity {
                moco_client
                    .get_activity(&GetActivity { activity_id: a })
                    .await?
            } else {
                promp_activity_select(&moco_client, activity).await?
            };

            let date = if let Some(d) = date {
                d
            } else {
                print!("New date (YYYY-MM-DD) - Default '{}': ", activity.date);
                std::io::stdout().flush()?;

                let date = utils::read_line()?;
                if date.is_empty() {
                    activity.date.clone()
                } else {
                    date
                }
            };

            let hours = if let Some(h) = hours {
                h
            } else {
                print!("New duration (hours) - Default '{}': ", activity.hours);
                std::io::stdout().flush()?;

                let hours = utils::read_line()?;
                if hours.is_empty() {
                    activity.hours
                } else {
                    hours.parse::<f64>()?
                }
            };

            let description = if let Some(d) = description {
                d
            } else {
                print!("New description - Default 'current': ");
                std::io::stdout().flush()?;

                let description = utils::read_line()?;
                if description.is_empty() {
                    activity
                        .description
                        .as_ref()
                        .unwrap_or(&String::new())
                        .to_string()
                } else {
                    description
                }
            };

            moco_client
                .edit_activity(&EditActivity {
                    activity_id: activity.id,
                    project_id: Some(project).unwrap(),
                    task_id: task,
                    date,
                    hours: hours.to_string(),
                    description,
                })
                .await?;

            let activity2 = moco_client
                .get_activity(&GetActivity {
                    activity_id: activity.id,
                })
                .await?;

            let (from, to) = utils::select_from_to_date(true, false, false);

            let activities = moco_client
                .get_activities(
                    from.format("%Y-%m-%d").to_string(),
                    to.format("%Y-%m-%d").to_string(),
                )
            .await?;

            let today: Vec<f64> =  activities.iter().map(|activity| {
                activity.hours
            }).collect();

            let today_hours: f64 = today.iter().sum();

            let report = moco_client.get_performance_report().await?;

            let operator = if report.annually.variation_until_today > 0.00 {
                "+"
            } else {
                ""
            };

            let mut file = File::create("/home/rodox/.config/mococli/moco_report")?;
            file.write(
                format!(
                    "{}{} {}",
                    operator,
                    report.annually.variation_until_today.to_string(),
                    today_hours.to_string()
                )
                .as_bytes(),
            )?;

            println!(
                "Activity\n    Customer: {}\n    Project: {}\n    Task: {}\n    Duration: {} h\n    Description: {}",
                activity2.customer.name,
                activity2.project.name,
                activity2.task.name,
                activity2.hours,
                activity2.description.unwrap()
            )
        }
        cli::Commands::Rm { activity } => {
            let activity = moco_client
                .get_activity(&GetActivity {
                    activity_id: activity,
                })
                .await?;

            moco_client
                .delete_activity(&DeleteActivity {
                    activity_id: activity.id,
                })
                .await?;

            let (from, to) = utils::select_from_to_date(true, false, false);

            let activities = moco_client
                .get_activities(
                    from.format("%Y-%m-%d").to_string(),
                    to.format("%Y-%m-%d").to_string(),
                )
            .await?;

            let today: Vec<f64> =  activities.iter().map(|activity| {
                activity.hours
            }).collect();

            let today_hours: f64 = today.iter().sum();

            let report = moco_client.get_performance_report().await?;

            let operator = if report.annually.variation_until_today > 0.00 {
                "+"
            } else {
                ""
            };

            let mut file = File::create("/home/rodox/.config/mococli/moco_report")?;
            file.write(
                format!(
                    "{}{} {}",
                    operator,
                    report.annually.variation_until_today.to_string(),
                    today_hours.to_string()
                )
                .as_bytes(),
            )?;

            println!(
                "Activity\n    Project: {}\n    Task: {}\n    Duration: {} h\n    Description: {}",
                activity.project.name,
                activity.task.name,
                activity.hours,
                activity.description.unwrap()
            )
        }
        cli::Commands::Timer { system } => match system {
            cli::Timer::Start { activity } => {
                let activity = if let Some(a) = activity {
                    moco_client
                        .get_activity(&GetActivity { activity_id: a })
                        .await?
                } else {
                    promp_activity_select(&moco_client, activity).await?
                };

                moco_client
                    .control_activity_timer(&ControlActivityTimer {
                        control: "start".to_string(),
                        activity_id: activity.id,
                    })
                    .await?;

                let activity = moco_client
                    .get_activity(&GetActivity {
                        activity_id: activity.id,
                    })
                    .await?;

                let duration = Duration::minutes((activity.hours * 60.0) as i64);

                let mut file = File::create("/home/rodox/.config/mococli/moco_timer")?;
                file.write(
                    format!(
                        "{} {}",
                        Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        (Local::now() - duration)
                            .format("%Y-%m-%d %H:%M:%S")
                            .to_string()
                    )
                    .as_bytes(),
                )?;

                println!(
                    "Activity\n    Project: {}\n    Task: {}\n    Duration: {} h\n    Description: {}",
                    activity.project.name,
                    activity.task.name,
                    activity.hours,
                    activity.description.unwrap()
                )
            }
            cli::Timer::Stop => {
                remove_file("/home/rodox/.config/mococli/moco_timer")
                    .expect("Timer file did not exist");

                let now = Local::now().format("%Y-%m-%d").to_string();
                let from = now.clone();
                let to = now.clone();

                let activities = moco_client.get_activities(from, to).await?;
                let activity = activities.iter().find(|a| !a.timer_started_at.is_null());

                if let Some(a) = activity {
                    moco_client
                        .control_activity_timer(&ControlActivityTimer {
                            control: "stop".to_string(),
                            activity_id: a.id,
                        })
                        .await?;

                    let a = moco_client
                        .get_activity(&GetActivity { activity_id: a.id })
                        .await?;

                    let (from, to) = utils::select_from_to_date(true, false, false);

                    let activities = moco_client
                        .get_activities(
                            from.format("%Y-%m-%d").to_string(),
                            to.format("%Y-%m-%d").to_string(),
                        )
                    .await?;

                    let today: Vec<f64> =  activities.iter().map(|activity| {
                        activity.hours
                    }).collect();

                    let today_hours: f64 = today.iter().sum();

                    let report = moco_client.get_performance_report().await?;

                    let operator = if report.annually.variation_until_today > 0.00 {
                        "+"
                    } else {
                        ""
                    };

                    let mut file = File::create("/home/rodox/.config/mococli/moco_report")?;
                    file.write(
                        format!(
                            "{}{} {}",
                            operator,
                            report.annually.variation_until_today.to_string(),
                            today_hours.to_string()
                        )
                        .as_bytes(),
                    )?;

                    println!(
                        "Activity\n    Project: {}\n    Task: {}\n    Duration: {} h\n    Description: {}",
                        a.project.name,
                        a.task.name,
                        a.hours,
                        a.description.unwrap()
                    );
                } else {
                    println!("Timer was already stopped...");
                }
            }
        },
        cli::Commands::Customers {} => {
            let projects = moco_client.get_assigned_projects().await?;

            let mut list: Vec<Vec<String>> = projects
                .into_iter()
                .unique_by(|project| project.customer.id)
                .map(|project| {
                    vec![
                        project.customer.id.to_string(),
                        project.customer.name.clone(),
                    ]
                })
                .collect();
            list.insert(0, vec!["ID".to_string(), "Name".to_string()]);

            list.push(vec!["-".to_string(), "-".to_string()]);

            render_table(list)
        }
        cli::Commands::Projects { customer } => {
            let projects = moco_client.get_assigned_projects().await?;

            let mut list: Vec<Vec<String>> = projects
                .iter()
                .filter(|project| project.customer.id == customer.unwrap())
                .map(|project| {
                    vec![
                        project.id.to_string(),
                        project.name.clone(),
                        project.customer.name.clone(),
                    ]
                })
                .collect();

            list.insert(
                0,
                vec!["ID".to_string(), "Name".to_string(), "Customer".to_string()],
            );

            list.push(vec!["-".to_string(), "-".to_string(), "-".to_string()]);

            render_table(list)
        }
        cli::Commands::Tasks { project } => {
            let tasks = moco_client
                .get_project_tasks(&GetProjectTasks {
                    project_id: project,
                })
                .await?;

            let mut list: Vec<Vec<String>> = tasks
                .iter()
                .map(|task| vec![task.id.to_string(), task.name.clone()])
                .collect();

            list.insert(0, vec!["ID".to_string(), "Name".to_string()]);

            list.push(vec!["-".to_string(), "-".to_string()]);

            render_table(list)
        }
        cli::Commands::Activity { activity } => {
            let activity = moco_client
                .get_activity(&GetActivity {
                    activity_id: activity,
                })
                .await?;

            println!(
                "\n{}\n{}\n{}\n{}\n{}\n{}",
                activity.customer.name,
                activity.project.name,
                activity.task.name,
                activity.date,
                activity.hours,
                activity
                    .description
                    .as_ref()
                    .unwrap_or(&String::new().to_string())
            )
        }
        cli::Commands::Activities { from, to } => {
            let activities = moco_client.get_activities(from, to).await?;

            let mut list: Vec<Vec<String>> = activities
                .iter()
                .enumerate()
                .map(|(index, activity)| {
                    vec![
                        activity.id.to_string(),
                        format!(
                            "{} - {} | {} | {} | {} h | {}",
                            index.to_string(),
                            activity.customer.name.clone(),
                            activity.project.name.clone(),
                            activity.task.name.clone(),
                            activity.hours.to_string(),
                            activity
                                .description
                                .as_ref()
                                .unwrap_or(&String::new().to_string())
                        ),
                    ]
                })
                .collect();
            list.insert(0, vec!["ID".to_string(), "Information".to_string()]);

            list.push(vec!["-".to_string(), "".to_string()]);

            render_table(list)
        }

        cli::Commands::Report {} => {
            let (from, to) = utils::select_from_to_date(true, false, false);

            let activities = moco_client
                .get_activities(
                    from.format("%Y-%m-%d").to_string(),
                    to.format("%Y-%m-%d").to_string(),
                )
                .await?;

            let today: Vec<f64> =  activities.iter().map(|activity| {
                activity.hours
            }).collect();

            let today_hours: f64 = today.iter().sum();

            let report = moco_client.get_performance_report().await?;

            let operator = if report.annually.variation_until_today > 0.00 {
                "+"
            } else {
                ""
            };

            let mut file = File::create("/home/rodox/.config/mococli/moco_report")?;
            file.write(
                format!(
                    "{}{} {}",
                    operator,
                    report.annually.variation_until_today.to_string(),
                    today_hours.to_string()
                )
                .as_bytes(),
            )?;

            println!(
                "{}{} {}",
                operator,
                report.annually.variation_until_today.to_string(),
                today_hours.to_string()
            )
        }
    }

    Ok(())
}
