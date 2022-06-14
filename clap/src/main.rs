use clap::{App};
use std::fs;

pub struct CLIApplication {
    name: String,
    cli: App<'static, 'static>,
}

impl CLIApplication {
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn context(yaml_path: &str) -> App<'static, 'static> {
    let command_configuration: String = match fs::read_to_string(yaml_path) {
        Ok(s) => String::from(s),
        Err(e) => panic!("unable to open configuration file: {}", e),
    };  
    
    let yml_docs = match clap::YamlLoader::load_from_str(&command_configuration) {
        Ok(yml) => yml,
        Err(e)=> panic!("Unable to parse yaml configuration: {}", e),
    };

    print_type_of(&yml_docs);
    let document = &yml_docs[0];
    print_type_of(&document);
    let app_name = document["name"].as_str().unwrap();
    println!("{:?}",document);

    App::from_yaml(&yml_docs[0])
}

fn main() {
    let cli = context("../commands.yml");
}
