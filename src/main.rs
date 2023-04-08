extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use std::env::current_dir;
use std::collections::HashMap;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct SystemdParser;

/// Implement a simple AST representation
#[derive(Debug, Clone)]
pub enum SystemdValue {
    List(Vec<String>),
    Str(String),
}


fn pre_process_map(map: &mut HashMap<String, HashMap<String, SystemdValue>>) {
    for (_, value) in map.into_iter() {
        for (_, v) in value.into_iter() {
            if let SystemdValue::List(vs) = v {
                if vs.len() == 0 {
                    let v_ = SystemdValue::Str(String::new());
                    *v = v_.clone();
                } else if vs.len() == 1 {
                    let v_ = SystemdValue::Str((vs[0]).clone());
                    *v = v_.clone();
                }
            }
        }
    }
}

fn main() {
    // Read and parse the unit file.
    let unparsed_file = fs::read_to_string("app.service")
        .expect("cannot read file");
    let file = SystemdParser::parse(Rule::file, &unparsed_file).expect("fail to parse")
        .next()
        .unwrap();

    // Create a fresh HashMap to store the data.
    let mut properties: HashMap<String, HashMap<String, SystemdValue>> = HashMap::new();

    // These two mutable variables will be used to store
    // section name and property key name.
    let mut current_section_name = String::new();
    let mut current_key_name = String::new();

    // Iterate over the file line-by-line.
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::section => {
                // Update the current_section_name
                let mut inner_rules = line.into_inner();
                current_section_name = inner_rules.next().unwrap().as_str().to_string();
            }
            Rule::property => {
                let mut inner_rules = line.into_inner();
                // Get a sub map of properties with the current_section_name key, or create new.
                let section = properties.entry(current_section_name.clone()).or_default();

                // Get the current property name and value.
                let name = inner_rules.next().unwrap().as_str().to_string();
                let value = inner_rules.next().unwrap().as_str().to_string();

                // If the property name already exists...
                if name == current_key_name {
                    // Get the section of the map with the key name, or insert a new SytemdValue::List.
                    let entry = section.entry(current_key_name.clone()).or_insert(SystemdValue::List(vec![]));
                    // Push the value onto the inner vector of SystemdValue::List.
                    if let SystemdValue::List(ent) = entry {
                        ent.push(value);
                    }
                } else {
                    // Create a new SystemdValue::List and save it under name key.
                    let entry = section.entry(name.clone()).or_insert(SystemdValue::List(vec![]));
                    // Push the current value onto the vector, then set the
                    // current_key_name to the current name.
                    if let SystemdValue::List(ent) = entry {
                        ent.push(value);
                    }
                    current_key_name = name;
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    pre_process_map(&mut properties);

    println!("{:#?}", properties);
    
}
