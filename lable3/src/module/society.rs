use super::student::Student;
#[derive(Debug, Clone)]
pub struct Society {
    pub name: String,
    pub members: Vec<Student>,
}

impl Society {
    pub fn change_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn add_student(&mut self, student: Student) {
        self.members.push(student);
    }

    pub fn remove_student(&mut self, name: String) -> bool {
        if let Some(index) = self.members.iter().position(|s| s.name == name) {
            self.members.remove(index);
            return true;
        }
        false
    }
}