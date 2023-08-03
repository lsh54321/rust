use lable3::module::{student::{Student, self}, class::{Class, Grade}, course::Course, society::Society};

fn main() {
    let mut all_students: Vec<Student> = vec![];

    let sofia = Student::new(String::from("Sofia"), 10);
    let lucy = Student::new(String::from("Lucy"), 12);
    let macks = Student::new(String::from("Macks"), 11);

    // init all students
    all_students.push(sofia);
    all_students.push(lucy);
    all_students.push(macks);
    all_students.push(Student::new(String::from("Pack"), 15));

    // ------ Student CURD begin -----
    // query student with name `foo`
    let foo = student::find_first_by_name(&all_students, String::from("foo"));
    assert!(foo.is_none(), "find foo student");
    let sofia = student::find_first_by_name(&all_students, String::from("Sofia"));
    assert_eq!(sofia.unwrap().0.age, 10);
    let index = sofia.unwrap().1;
    let _ = &mut (all_students[index].increase_age());

    //remove foo all students
    assert!(
        !student::remove_first_by_name(&mut all_students, String::from("foo")),
        "remove foo student"
    );
    assert!(
        student::remove_first_by_name(&mut all_students, String::from("Lucy")),
        "cant't remove tom student"
    );
    assert_eq!(all_students.len(), 3);
    println!("{:?}", all_students);
    // ------ Student CURD end -----

    // ------ Class CURD begin -----
    let mut class1 = Class {
        grade: Grade::ONE,
        num: 1,
        students: vec![],
    };
    let sofia = student::find_first_by_name_clone(&all_students, String::from("Sofia")).unwrap();
    class1.add_student(sofia);
    let lucy = student::find_first_by_name_clone(&all_students, String::from("Lucy")).unwrap();
    class1.add_student(lucy);
    println!("{:?}", class1);
    class1.remove_student(String::from("Lucy"));
    println!("{:?}", class1);
    // ------ Class CURD end -----

    let mut couse1 = Course {
        name: String::from("English"),
        students: vec![],
    };
    let sofia = student::find_first_by_name_clone(&all_students, String::from("Sofia")).unwrap();
    couse1.add_student(sofia);
    println!("{:?}", couse1);

    let mut society = Society {
        name: String::from("Game"),
        members: vec![],
    };
    let sofia = student::find_first_by_name_clone(&all_students, String::from("Sofia")).unwrap();
    society.add_student(sofia);
    println!("{:?}", society);
}