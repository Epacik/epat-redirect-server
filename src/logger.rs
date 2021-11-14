use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use log::{Record, Level, Metadata};
pub struct Logger;


impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }


        let mut now = chrono::offset::Local::now();

        let new_log = format!("[{}] {} - {}",
                              &now.format("%Y-%m-%d %H:%M:%S.%f"),
                              record.level(),
                              record.args());

        println!("{}", &new_log);

        if !Path::new("logs").exists() {
            let result = std::fs::create_dir("logs");
            if result.is_err()
            {
                return;
            }
        }

        let filename = &now.format("logs/%Y-%m-%d-%H.log").to_string();
        let file_result = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(filename);

        if file_result.is_err(){
            return;
        }
        let mut file = file_result.unwrap();

        let write_file_result = file.write(format!("{}\n", new_log).as_ref());
        if write_file_result.is_err(){
            now = chrono::offset::Local::now();

            println!("[{}] ERROR - Could not append line to file\n{}",
                     &now.format("%Y-%m-%d %H:%M:%S.%f"),
                     write_file_result.unwrap_err());
        }
    }

    fn flush(&self) {}
}