use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

// Employment

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Employment {
    pub id: i64,
    #[serde(rename = "weekly_target_hours")]
    pub weekly_target_hours: f64,
    pub pattern: Pattern,
    pub from: String,
    pub to: Value,
    pub user: User,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pattern {
    pub am: Vec<f64>,
    pub pm: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
}

// Activity

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub id: i64,
    pub date: String,
    pub hours: f64,
    pub seconds: i64,
    pub description: Option<String>,
    pub billed: bool,
    pub billable: bool,
    pub tag: String,
    #[serde(rename = "remote_service")]
    pub remote_service: Option<String>,
    #[serde(rename = "remote_id")]
    pub remote_id: Option<String>,
    #[serde(rename = "remote_url")]
    pub remote_url: Value,
    pub project: ActivityProject,
    pub task: Task,
    pub customer: Customer,
    pub user: User,
    #[serde(rename = "timer_started_at")]
    pub timer_started_at: Value,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "hourly_rate")]
    pub hourly_rate: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityProject {
    pub id: i64,
    pub name: String,
    pub billable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: i64,
    pub name: String,
    pub billable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProjectTasks {
    pub project_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetActivity {
    #[serde(rename = "activity_id")]
    pub activity_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateActivity {
    pub date: String,
    pub description: String,
    #[serde(rename = "project_id")]
    pub project_id: i64,
    #[serde(rename = "task_id")]
    pub task_id: i64,
    pub hours: Option<f64>,
    pub seconds: Option<i64>,
    pub tag: Option<String>,
    #[serde(rename = "remote_service")]
    pub remote_service: Option<String>,
    #[serde(rename = "remote_id")]
    pub remote_id: Option<String>,
    #[serde(rename = "remote_url")]
    pub remote_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditActivity {
    #[serde(rename = "activity_id")]
    pub activity_id: i64,
    #[serde(rename = "project_id")]
    pub project_id: i64,
    #[serde(rename = "task_id")]
    pub task_id: i64,
    pub date: String,
    pub description: String,
    pub hours: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControlActivityTimer {
    pub control: String,
    #[serde(rename = "activity_id")]
    pub activity_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteActivity {
    #[serde(rename = "activity_id")]
    pub activity_id: i64,
}

// Project

pub type Projects = Vec<Project>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: i64,
    pub identifier: String,
    pub name: String,
    pub active: bool,
    pub billable: bool,
    pub customer: Customer,
    pub tasks: Vec<ProjectTask>,
}

// Task

pub type ProjectTasks = Vec<Task>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectTask {
    pub id: i64,
    pub name: String,
    pub active: bool,
    pub billable: bool,
}

// Customer

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub id: i64,
    pub name: String,
}

// User

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceReport {
    pub annually: AnnualReport,
    pub monthly: Vec<MonthlyReport>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnualReport {
    pub year: i32,
    #[serde(rename = "employment_hours")]
    pub employment_hours: f64,
    #[serde(rename = "target_hours")]
    pub target_hours: f64,
    #[serde(rename = "hours_tracked_total")]
    pub hours_tracked_total: f64,
    pub variation: f64,
    #[serde(rename = "variation_until_today")]
    pub variation_until_today: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyReport {
    pub year: i32,
    pub month: i32,
    #[serde(rename = "target_hours")]
    pub target_hours: f64,
    #[serde(rename = "hours_tracked_total")]
    pub hours_tracked_total: f64,
    pub variation: f64,
}
