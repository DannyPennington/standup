use config::{Config, ConfigError};
use std::env;

pub fn basic_grouping(people: Vec<String>) -> Vec<Vec<String>> {
    let mut grouped_people: Vec<Vec<String>> = vec![];
    let mut buffer: Vec<String> = vec![];
    let limit = people.len() - 1;
    for (index, person) in people.into_iter().enumerate() {
        buffer.push(person);
        if buffer.len() == 4 || index == limit {
            grouped_people.push(buffer.clone());
            buffer.clear()
        }
    }
    grouped_people
}

pub fn role_grouping(people: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut result = vec![];
    for group in people {
        result.push(basic_grouping(group))
    }
    result.into_iter().flatten().collect()
}

pub fn determine_config() -> Result<Config, ConfigError> {
    let mut config = config::Config::default();
    /*
    let args: Vec<String> = std::env::args().collect();
    let name = match args.len() {
        0 | 1 => {"nw"},
        _ => {&args[1]}
    };
     */
    let name = env::var("TEAM").unwrap_or_else(|_| "nw".to_owned());
    let base = "./config/";
    config.merge(config::File::with_name(&*(base.to_owned() + &name)))?;
    Ok(config)
}
