#[derive(Queryable)]
pub struct TodoEntry {
    pub id: i32,
    pub text: String,
}