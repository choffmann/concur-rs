use std::{env, process::{Command, Stdio}, sync, thread::{self, JoinHandle}};

mod logger;
use logger::{LogColor, Logger};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let colors = vec![LogColor::Yellow, LogColor::Blue, LogColor::Magenta];

    let (tx, rx) = sync::mpsc::channel();
    let mut handler: Vec<Option<JoinHandle<()>>> = args.iter().enumerate().map(|(i, arg)| {
        let (arg1, arg2) = if cfg!(windows) { ("cmd", "/C") } else { ("sh", "-c") };

        let mut cmd = Command::new(arg1)
            .args([arg2, arg])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to prepare command");

        let tx = tx.clone();
        let color = colors[i%3].clone();
        let log = Logger::new(format!("Process {i}"), color);

        let stdout = cmd.stdout.take().expect("failed to capture stdout");
        let stderr = cmd.stderr.take().expect("failed to capture stderr");
        let log_handler = log.stream(stdout, None);
        let elog_handler = log.stream(stderr, Some(LogColor::Red));

        let h = thread::spawn(move || {
            cmd.wait_with_output().expect("failed to execute command");
            tx.send(i).unwrap();
            log_handler.join().unwrap();
            elog_handler.join().unwrap();
        });

        Some(h)
    }).collect();

    drop(tx);

    loop {
        let num_left = handler.iter().filter(|h| h.is_some()).count();
        if num_left == 0 {
            break;
        }

        let i = rx.recv().unwrap();
        let join_handler = std::mem::take(&mut handler[i]).unwrap();
        join_handler.join().unwrap();
    };
}
