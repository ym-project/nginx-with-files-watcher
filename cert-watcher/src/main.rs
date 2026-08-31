use notify_debouncer_mini::{DebounceEventResult, new_debouncer, notify::RecursiveMode};
use std::{env, path::Path, process::Command, thread::sleep, time::Duration};

fn main() {
	let cert_dir =
		env::var("CERT_WATCHER_DIR").expect("env variable `CERT_WATCHER_DIR` is required");
	let debounce_time = Duration::from_secs(5);
	let loop_interval = Duration::from_secs(60);

	println!("Starting certificate watcher for {}", cert_dir);

	let mut debouncer = new_debouncer(debounce_time, |result: DebounceEventResult| {
		match result {
			Ok(events) => {
				println!("Detected {} changes", events.len());

				let command_result = Command::new("nginx").args(["-s", "reload"]).status();

				if let Ok(exit_status) = command_result
					&& exit_status.success()
				{
					println!("Nginx reloaded successfully");
				} else {
					eprintln!("Nginx reload failed");
				}
			},
			Err(err) => eprintln!("Watch error: {:?}", err),
		};
	})
	.unwrap();

	// Watch changes
	debouncer.watcher().watch(Path::new(&cert_dir), RecursiveMode::Recursive).unwrap();

	// Keep main thread alive
	loop {
		sleep(loop_interval);
	}
}
