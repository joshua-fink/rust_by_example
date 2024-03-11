// Rust does not have inheritance -> but traits can be defined as superset of another trait

trait Person {
    fn name(&self) -> String;
}

// Person is supertrait of Student
// Implementing Student requires implementation of Person
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent is subtrait of both Programmer and Student
// Implementing CompSciTrait requires impl both supertraits
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My GitHub is {}.",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username(),
    )
}

fn main() {}