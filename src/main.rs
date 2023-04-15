use std::collections::HashMap;
use std::io::{self, BufRead};
use colored::Colorize;

fn main() {
    println!("Enter the content of the first .env file (type END and press enter to finish):");
    let env1_content = read_multiline_input();
    let env1 = parse_env(&env1_content);

    println!("Enter the content of the second .env file (type END and press enter to finish):");
    let env2_content = read_multiline_input();
    let env2 = parse_env(&env2_content);
    compare_envs(&env1, &env2);
}

fn read_multiline_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let sentinel_value = "END";

    loop {
        let mut line = String::new();
        let _ = stdin_lock.read_line(&mut line).unwrap();
        if line.trim() == sentinel_value {
            break;
        }
        input.push_str(&line);
    }

    input
}

fn compare_envs(env1: &HashMap<String, String>, env2: &HashMap<String, String>) {
    let mut missing_from_env1: Vec<&String> = Vec::new();
    let mut missing_from_env2: Vec<&String> = Vec::new();
    let mut different_values: Vec<(&String, &String, &String)> = Vec::new();

    for (key, value) in env1 {
        if let Some(env2_value) = env2.get(key) {
            if value != env2_value {
                different_values.push((key, value, env2_value));
            }
        } else {
            missing_from_env2.push(key);
        }
    }

    for key in env2.keys() {
        if !env1.contains_key(key) {
            missing_from_env1.push(key);
        }
    }

    if !missing_from_env1.is_empty() {
        println!("First file missing variables: {:?}", missing_from_env1);
    }
    if !missing_from_env2.is_empty() {
        println!("Second file missing variables: {:?}", missing_from_env2);
    }
    if !different_values.is_empty() {
        println!("Variables with different values:");
        for (key, value1, value2) in different_values {
            println!(
                "{}: {} -> {}",
                key,
                value1.green(),
                value2.red()
            );
        }
    }
}

fn parse_env(content: &str) -> HashMap<String, String> {
    let mut env = HashMap::new();

    for line in content.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        if let Some(eq_pos) = line.find('=') {
            let key = &line[..eq_pos].trim();
            let value = &line[(eq_pos + 1)..].trim();

            env.insert(key.to_string(), value.to_string());
        }
    }

    env
}
