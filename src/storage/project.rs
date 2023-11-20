use super::Store;

pub type ProjectStore = Store<Project, ()>;

#[derive(Default, Debug)]
pub struct Project {
    name: String,
}
