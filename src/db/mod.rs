use std::fmt;
use std::error;

use uuid::Uuid;
use crate::models::project::Project;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct NoProjects;

impl fmt::Display for NoProjects {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No projects available for this user id")
    }
}

impl error::Error for NoProjects {}

pub trait Database<T> {
    fn new(config: T) -> Self;
    fn get_projects_from_user_id(user_id: Uuid) -> Result<Vec<Project>>;
    fn create_project(user_id: Uuid) -> Result<Uuid>;
}
