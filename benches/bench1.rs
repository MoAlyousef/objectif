use objectif::{call_method, inherits, super_init, table_init, Object};

pub struct Student {
    pub age: u32,
}

impl Student {
    fn new(age: u32) -> Self {
        Self { age }
    }
}

impl HasAge for Student {
    fn age(&self) -> u32 {
        self.age
    }
}

pub struct MedStudent {
    parent: Student,
}

impl MedStudent {
    fn new(age: u32) -> Self {
        Self {
            parent: Student::new(age),
        }
    }
}

impl std::ops::Deref for MedStudent {
    type Target = Student;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl std::ops::DerefMut for MedStudent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

pub trait HasAge {
    fn age(&self) -> u32;
}

impl HasAge for MedStudent {
    fn age(&self) -> u32 {
        self.age
    }
}

#[inherits(Object)]
struct StudentObj {
    parent: Object,
    age: u32,
}

impl StudentObj {
    pub fn new(age: u32) -> Self {
        table_init!(
            StudentObj,
            "age": age,
        );
        Self {
            parent: super_init![Object::default()],
            age,
        }
    }
    pub fn age(&self) -> u32 {
        self.age
    }
}

#[inherits(StudentObj)]
struct MedStudentObj {
    parent: StudentObj,
}

impl MedStudentObj {
    fn new(age: u32) -> Self {
        table_init!(MedStudentObj,);
        Self {
            parent: super_init![StudentObj::new(age)],
        }
    }
}

fn benchmark(count: u32) {
    dbg!(std::mem::size_of::<MedStudentObj>());
    dbg!(std::mem::size_of::<Box<MedStudentObj>>());
    dbg!(std::mem::size_of::<MedStudent>());
    dbg!(std::mem::size_of::<Box<dyn HasAge>>());
    let mut sum: u32 = 0;
    let mut v: Vec<Box<StudentObj>> = vec![];
    for i in 0..count {
        if i % 2 == 0 {
            let medstudentobj = MedStudentObj::new(24);
            v.push(unsafe { std::mem::transmute(Box::new(medstudentobj)) });
        } else {
            let studentobj = StudentObj::new(20);
            v.push(Box::new(studentobj));
        }
    }
    let start = std::time::Instant::now();
    for elem in v {
        let val: u32 = unsafe { call_method![*elem, age].unwrap() };
        sum += val;
    }
    let duration = start.elapsed();
    let average = sum / count;
    println!(
        "value {average}: dyndispatch Object {} nanos per call",
        duration.as_nanos() / count as u128
    );

    let mut v: Vec<Box<dyn HasAge>> = vec![];
    for i in 0..count {
        if i % 2 == 0 {
            let medstudent = MedStudent::new(24);
            v.push(Box::new(medstudent));
        } else {
            let student = Student::new(20);
            v.push(Box::new(student));
        }
    }
    sum = 0;
    let start = std::time::Instant::now();
    for elem in v {
        let val: u32 = elem.age();
        sum += val;
    }
    let duration = start.elapsed();
    let average = sum / count;
    println!(
        "value {average}: dyndispatch Trait {} nanos per call",
        duration.as_nanos() / count as u128
    );
}

fn main() {
    let count = {
        let args: Vec<_> = std::env::args().collect();
        if args.len() == 1 {
            100000
        } else {
            args[1].parse().unwrap_or(100000)
        }
    };
    std::hint::black_box(benchmark(count));
}
