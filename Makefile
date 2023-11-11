main: main.rs libcommand.so
	rustc -L . main.rs

libcommand.so: command.rs
	rustc --crate-type proc-macro command.rs
