use std::{env, io::{BufRead, BufReader}, process::{Command, Stdio}, sync, thread::{self, JoinHandle}};

mod logger;
use logger::Logger;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let (tx, rx) = sync::mpsc::channel();
    let mut handler: Vec<Option<JoinHandle<()>>> = args.iter().enumerate().map(|(i, arg)| {
        let (arg1, arg2) = if cfg!(windows) { ("cmd", "/C") } else { ("sh", "-c") };

        let mut cmd = Command::new(arg1)
            .args([arg2, arg])
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to prepare command");

        let tx = tx.clone();
        let h = thread::spawn(move || {
            let log = Logger::new("foo".to_string());
            let stdout = cmd.stdout.as_mut().unwrap();
            let stdout_reader = BufReader::new(stdout);
            let stdout_lines = stdout_reader.lines();

            for line in stdout_lines {
                log.println(line.unwrap()).unwrap();
            }

            cmd.wait_with_output().expect("failed to execute command");
            tx.send(i).unwrap();
        });

        Some(h)
    }).collect();

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
