//This was going to be a class for Student

mod student_enrolment{
    pub struct Student{
        pub fname: String,
        pub lname: String,
        pub year: String,
    }


    impl Student{
        pub fn register(fname: &str, lname: &str, year: &str) -> Student{
            Student{
                fname: String::from(fname),
                lname: String::from(lname),
                year: String::from(year),
            }
        }
    }

    pub struct TakingClass{
        pub Student,
        pub class_name: String,
        pub class_number: i32,
        pub grade: String,
    }


    impl TakingClass{
        pub fn enroll(Student, class_name: &str, class_number: &str, grade: &str) -> TakingClass{
            TakingClass{
                Student,
                class_name: String::from(class_name),
                class_number: String::from(class_number),
                grade: String::from(grade),
            }
        }
    }

}