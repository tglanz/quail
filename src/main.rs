use quail::*;

fn main() {
    let log_path = "./assets/log4rs.yaml";
    if let Err(error) = log4rs::init_file(log_path, Default::default()) {
        println!("failed to load log configuration from {}. error: {}", log_path, error);
        return;
    }

    application::run();
}