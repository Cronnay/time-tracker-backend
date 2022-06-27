pub struct ProjectsController {
}

#[get("/<user_id>/projects")]
pub fn get_projects(user_id: &str) -> String {
    "".to_string()
}
