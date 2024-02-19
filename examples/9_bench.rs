use objectif::{call_method, inherits, init_table, super_init, Object};

pub struct Student {
    pub age: u32,
}

impl Default for Student {
    fn default() -> Self {
        Self {
            age: 20,
        }
    }
}

impl HasAge for Student {
    fn age(&self) -> u32 {
        self.age
    }
}

pub struct MedStudent {
    parent: Student,
    age: u32,
}

impl Default for MedStudent {
    fn default() -> Self {
        Self {
            parent: Student::default(),
            age: 23,
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

impl Default for StudentObj {
    fn default() -> Self {
        init_table!(
            StudentObj,
            "age": age,
        );
        Self {
            parent: super_init![Object::default()],
            age: 20,
        }
    }
}

impl StudentObj {
    pub fn age(&self) -> u32 {
        self.age
    }
}

#[inherits(StudentObj)]
struct MedStudentObj {
    parent: StudentObj,
    age: u32,
}

impl Default for MedStudentObj {
    fn default() -> Self {
        init_table!(
            MedStudentObj,
            "age":age,
        );
        Self {
            parent: super_init![StudentObj::default()],
            age: 23,
        }
    }
}

impl MedStudentObj {
    pub fn age(&self) -> u32 {
        self.age
    }
}

fn benchmark(count: u32) {
    let mut sum: u32 = 0;
    let mut v: Vec<Box<StudentObj>> = vec![];
    for _i in 0..count {
        let medstudentobj = MedStudentObj::default();
        v.push(unsafe { std::mem::transmute(Box::new(medstudentobj)) });
    }
    let start = std::time::Instant::now();
    for elem in v {
        let val: u32 = unsafe { call_method![*elem, age].unwrap() };
        sum += val;
    }
    let duration = start.elapsed();
    let average = sum/count;
    println!(
        "value {average}: dyndispatch Object {} nanos per call",
        duration.as_nanos() / count as u128
    );

    let mut v: Vec<Box<dyn HasAge>> = vec![];
    for _i in 0..count {
        let medstudent = MedStudent::default();
        v.push(Box::new(medstudent));
    }
    sum = 0;
    let start = std::time::Instant::now();
    for elem in v {
        let val: u32 = elem.age();
        sum += val;
    }
    let duration = start.elapsed();
    let average = sum/count;
    println!(
        "value {average}: dyndispatch Trait {} nanos per call",
        duration.as_nanos() / count as u128
    );
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    std::hint::black_box(benchmark(args[1].parse().unwrap()));
}