use phf::{phf_map};

static STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

static PLANTS: phf::Map<char, &'static str> =  phf_map!{
    'V' => "violets",
    'R' => "radishes",
    'C' => "clover",
    'G' => "grass",
};

pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    let index = STUDENTS.iter().position(|&s| s == _student).unwrap() * 2;
    let mut result: Vec<&str> = Vec::new();
    let lines = _diagram.lines();
    for line in lines{
        for letter in &line.as_bytes()[index..index+2]{
            let plant_char = *letter as char;
            result.push(PLANTS.get(&plant_char).unwrap());
        }
    }
    result
}
