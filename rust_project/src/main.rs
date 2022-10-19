/*
My first thought was that I was going to wirte a program
that would let you save a file then access it again, and share it etc.
truns out that was a lot more than I was able to even figrue out
given the time frame I had to work with.
unfortunantoy I spen a long time studying and working on how I might
be able to make that happen.  It never did.  I then changed my mind 
I was going to write a program that would let you have students,
and classes with grades just so that I had something to turn in
It also never happened.  I have spent way too long working on this
with nothing to show for it.  I feel like I should have started
working on my second project earlier. */


use std::io;
//use rand::Rng;
use std::io::{Write, BufReader, BufRead};
use std::fs::File;
//use std::cmp::Ordering;

fn main() {

    //this is a constructor for my student was going to be class
    struct Student{
        pub fname: String,
        pub lname: String,
        pub year: String,
    }
    
    //This is where student was to be implimented it works but not well.
    impl Student{
        pub fn register(fname: &str, lname: &str, year: &str) -> Student{
            Student{
                fname: String::from(fname),
                lname: String::from(lname),
                year: String::from(year),
            }
        }
    }


    // this was going to be the takingclass class so that I could have students take more than one class
    /*struct TakingClass{
        pub student: &Student,
        pub class_name: String,
        pub class_number: String,
        pub grade: String,
    }

    impl TakingClass{
        pub fn enroll(student: &Student, class_name: &str, class_number: &str, grade: &str) -> TakingClass{
            TakingClass{
                student: &Student,
                class_name: String::from(class_name),
                class_number: String::from(class_number),
                grade: String::from(grade),
            }
        }
    }*/


    //This is where I was testing creating a student instance from the constructor
    let mut s1 = Student{fname: String::from("Chais"), lname: String::from("Chang"), year: String::from("Senior")};

    println!("Student First Name: {}", s1.fname);
    println!("Student Last Name: {}", s1.lname);
    println!("Student Year: {}", s1.year);

    //This is where I was testing creating a student instance from the register function
    let mut s2 = Student::register("Chais", "Chang", "Senior");

    println!("Student First Name: {}", s2.fname);
    println!("Student Last Name: {}", s2.lname);
    println!("Student Year: {}", s2.year);

    //let mut e1 = TakingClass::enroll(&s1, "Applied Programming", "CSE 310", "A");


    //This is practice coce I got from working on a tutorial here is the link -- https://www.youtube.com/watch?v=ygL_xcavzQ4 --
    // this code creates an array of 9 32 bit integers
    let arr_2:[i32; 9] = [1,2,3,4,5,6,7,8,9];

    //This code lets you loop through the array and pick out the odd values ending before 9
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx +=1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val :  {}", arr_2[loop_idx]);
        loop_idx +=1;
    }

    //This code is a while loop much better than the one above that siimply loops and prints values
    let mut i = 0;
    while i < arr_2.len(){
        println!("Arr : {}", arr_2[i]);
        i +=1;
    }

    //This was practice for creating an enum I was planning to use this 
    enum Day{
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool{
            match self{
                Day::Saturday | Day::Sunday => true,
                _=> false
            }
        }
    }

    let today:Day = Day::Saturday;
    match today{
        Day:: Monday => println!("Everyone hates Monday"),
        Day:: Tuesday => println!("You have YM/YW today"),
        Day:: Wednesday => println!("Hump day"),
        Day:: Thursday => println!("Home work today"),
        Day:: Friday => println!("Date night"),
        Day:: Saturday => println!("Weekend"),
        Day:: Sunday => println!("Weekend"),
    }

    println!("Is today the weekend {}", today.is_weekend());

    //This was practice for getting input from the user
    let mut input = String::new();

    println!("Write something cool if you think you can.");

    io::stdin().read_line(&mut input).expect("failed to read line");

    println!("lets take a look at what your wrote");

    println!("{}", input);


    //This was practice for writing to a file and dealing with errors
    let path: &str = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {panic!("Problem creating file : {:?}", error);
        }
    };

    write!(output, "Just some \nRandom words").expect("Failed to write to file");

    let input: File = File::open(path).unwrap();
    let buffered: BufReader<File> = BufReader::new(input);
    for line in buffered.lines(){
        println!("{}", line.unwrap());
    }

}
