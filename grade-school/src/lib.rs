use std::collections::BTreeMap; // using a hashmap doesn't quite make a difference here
use std::collections::BinaryHeap;

pub struct School {
    students: BTreeMap<u32, BinaryHeap<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            students: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let g = self.students.entry(grade).or_insert_with(BinaryHeap::new);
        // First option: perform a regular insertion inside a vec (thus keeping the list of students unsorted), and sort during grade()
        // Second option: perform a sorted insertion inside a vec
        /*
        let pos = g.binary_search(&student.to_string()).unwrap_or_else(|e| e);
        g.insert(pos, student.to_string()),
        */
        // Third option: use a binary heap
        g.push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.students
            .get(&grade)
            .and_then(|v| Some(v.clone().into_sorted_vec()))
        // Same as
        /*
        match self.students.get(&grade)  {
            Some(v) => Some(v.clone().into_sorted_vec()),
            None => None
        }*/
    }
}
