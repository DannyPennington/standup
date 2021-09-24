
pub fn basic_grouping(people: Vec<&str>) -> Vec<Vec<&str>> {
    let mut grouped_people: Vec<Vec<&str>> = Vec::new();
    let mut buffer = Vec::new();
    for index in 0..people.len() {
        buffer.push(people[index]);
        if buffer.len() == 4 || index == people.len() - 1 {
            grouped_people.push(buffer.clone());
            buffer.clear();
        }
    }
    grouped_people
}

pub fn role_grouping(people: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
    let mut result = Vec::new();
    for group in people {
        result.push(basic_grouping(group))
    };
    result.into_iter().flatten().collect()
}
