use std::{
    collections::HashSet,
    fs::{File, self},
    process::{Command},
    thread::sleep,
    time::Duration,
};

use log::{debug, error, info, trace, warn, Level, LevelFilter};
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode, WriteLogger,
};
use sysinfo::{ProcessExt, RefreshKind, System, SystemExt};

use crate::win::log_and_notify;

mod win;

const PATH_TO_LIST: &str = "paths.txt";

fn main() {
    let logger_config = ConfigBuilder::default()
        .set_max_level(LevelFilter::Debug)
        .set_location_level(LevelFilter::Off)
        .set_time_to_local(true)
        .build();

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            logger_config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            logger_config,
            File::create("ap.log").unwrap(),
        ),
    ])
    .unwrap();

    loop {
        let sys = System::new_with_specifics(RefreshKind::new().with_processes());
        if let Err(err) = check_processes(sys) {
            log_and_notify(err, Level::Error, false, 0);
        }
    }
}

fn check_processes(mut sys: System) -> anyhow::Result<()> {
    loop {
        info!("Obtaining paths from {:?}", PATH_TO_LIST);
        let paths_list_string = fs::read_to_string(PATH_TO_LIST)?;

        if paths_list_string.len() > 0 {
            let paths_list = paths_list_string.lines().collect::<HashSet<&str>>();
            info!("Crated path set :\n{:#?}", paths_list);

            sys.refresh_processes();
            for (pid, process) in sys.processes() {
                if paths_list.contains(&process.name()) {
                    let taskkill = Command::new("taskkill")
                        .args(["/F", "/PID", pid.to_string().as_str()])
                        .status();
                    match taskkill {
                        Ok(exit_status) => {
                            if exit_status.code() == Some(0) {
                                let msg = format!("Killed {:?}", process.name());
                                log_and_notify(msg, Level::Info, true, 0);
                            } else {
                                let msg = format!(
                                    "Tried killing {:?}, got error code {}",
                                    process.name(),
                                    exit_status
                                        .code()
                                        .map_or("<NONE>".to_string(), |s| s.to_string())
                                );
                                log_and_notify(msg, Level::Error, true, 0);
                            }
                        }
                        Err(err) => {
                            let msg = format!(
                                "Tried killing {:?}, got error (without code) :\n{:?}",
                                process.name(),
                                err
                            );
                            log_and_notify(msg, Level::Error, true, 0);
                        }
                    }
                }
            }
            sleep(Duration::from_millis(1000));
        } else {
            let msg = format!(
                "No path found in {path_to_list:?}. Please fill {path_to_list:?} with your paths.",
                path_to_list = PATH_TO_LIST
            );
            log_and_notify(msg, Level::Error, false, 0);
        }
    }
}
