use std::env;
use std::fs;
use std::thread;
use std::time;
use std::io::{self, Write};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let batteries: Vec<String> = args.drain(1..).collect();

    let mut battery_infos = update_battery_info(batteries.clone());

    loop {
        if chrono::Local::now().timestamp() % 30 == 0 {
            battery_infos = update_battery_info(batteries.clone());
        }
        let datetime = update_datetime_info();

        println!("{0} | {1}", battery_infos, datetime);
        io::stdout().flush().unwrap();

        let pause = time::Duration::from_secs(1);
        thread::sleep(pause);
    }
}

fn update_datetime_info() -> String {
    let now = chrono::Local::now();
    return now.format("%d %m %Y %H:%M:%S").to_string();
}

fn update_battery_info(batteries: Vec<String>) -> String {
    let mut battery_infos = String::new();
    
    for battery in batteries {
        let path = format!("/sys/class/power_supply/{0}", battery);
        let battery_info = build_battery_info(&path);

        if !battery_infos.is_empty() {
            battery_infos.push_str(" | ");
        }
        battery_infos.push_str(&battery_info);
    }
    return battery_infos;
}

fn build_battery_info(path: &str) -> String {
    let manufacturer = read_info(path, "manufacturer");
    let model_name = read_info(path, "model_name");  
    let capacity = read_info(path, "capacity");

    return format!("{0} {1}: {2}%", manufacturer, model_name, capacity);
}

fn read_info(path: &str, file: &str) -> String {
    let filename = format!("{0}/{1}", path, file);
    let error_message = format!("Unable to read battery {0}", file);

    let mut data = fs::read_to_string(filename).expect(&error_message);
    data.pop();
    
    return data;
}




