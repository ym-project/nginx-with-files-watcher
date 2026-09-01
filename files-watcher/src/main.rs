use notify_debouncer_full::{
	DebounceEventResult, DebouncedEvent, new_debouncer,
	notify::{EventKind, RecursiveMode, event::ModifyKind},
};
use std::{env, path::Path, process::Command, thread::sleep, time::Duration};

fn is_event_relevant(event: &DebouncedEvent) -> bool {
	matches!(
		event.kind,
		EventKind::Create(_)
			| EventKind::Remove(_)
			| EventKind::Modify(ModifyKind::Data(_))
			| EventKind::Modify(ModifyKind::Name(_))
	)
}

/// Parse comma-separated directory list
fn parse_directories(raw_directories: &str) -> Vec<&str> {
	raw_directories.split(',').map(|i| i.trim()).filter(|i| !i.is_empty()).collect()
}

fn main() {
	let raw_directories =
		env::var("WATCHER_DIRECTORIES").expect("env variable `WATCHER_DIRECTORIES` is required");
	let directories = parse_directories(&raw_directories);

	let debounce_time = Duration::from_secs(5);
	let loop_interval = Duration::from_secs(60);

	let mut debouncer = new_debouncer(debounce_time, None, |result: DebounceEventResult| {
		match result {
			Ok(events) => {
				// Track only certain events
				let relevant_events: Vec<&DebouncedEvent> =
					events.iter().filter(|event| is_event_relevant(event)).collect();

				if relevant_events.is_empty() {
					return;
				}

				println!("Detected {} changes", relevant_events.len());

				// Reload nginx
				let command_result = Command::new("nginx").args(["-s", "reload"]).status();

				if let Ok(exit_status) = command_result
					&& exit_status.success()
				{
					println!("Nginx reloaded successfully");
				} else {
					eprintln!("Nginx reload failed");
				}
			},
			Err(errors) => {
				for error in errors {
					eprintln!("Watch error: {error:?}");
				}
			},
		};
	})
	.unwrap();

	for directory in &directories {
		println!("Starting watcher for {}", directory);
		// Watch directory changes
		debouncer.watch(Path::new(directory), RecursiveMode::Recursive).unwrap();
	}

	// Keep main thread alive
	loop {
		sleep(loop_interval);
	}
}
