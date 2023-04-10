
static STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    let index = STUDENTS.iter().position(|&s| s == _student).unwrap() * 2;
    _diagram.lines()
        .flat_map(|line| {
            line[index..=index+1].chars()
            .map(|c| match c {
                'G' => "grass",
                'C' => "clover",
                'R' => "radishes",
                _ => "violets",
            })
        })
        .collect()
}
