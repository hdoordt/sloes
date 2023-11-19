use super::Store;

pub type ProjectStore = Store<Project, ()>;
pub struct Project {
    name: String,
}
