use chrono::prelude::*;
use uuid::Uuid;

pub struct Project {
    project_id: Uuid,
    time_spent: Vec<TimeSpent>,
}

pub struct TimeSpent {
    note: String,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>
}
