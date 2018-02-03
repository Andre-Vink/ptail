extern crate notify;

//use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
//use notify::op::WRITE;
//use std::sync::mpsc::channel;
//use std::path::PathBuf;
use std::env;
use std::path::Path;
use std::fs;

fn main() {
    let args :Vec<String> = env::args().collect();
    let dir_to_watch = if args.len() < 2 { "." } else { args.get(1).unwrap() };
    println!("Tailing files in directory [{}]...", dir_to_watch);

    let p = Path::new(dir_to_watch);
    let rd = fs::read_dir(p);
    match rd {
        Ok(rd) => read_directory(rd),
        Err(err) => println!("Dir no good: {:?}", err),
    }
    println!("Tailing ended.");

    // Create a channel to receive the events.
//    let (tx, rx) = channel();

    // Create a watcher object, delivering raw events.
    // The notification back-end is selected based on the platform.
//    let mut watcher = raw_watcher(tx).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
//    watcher.watch(".", RecursiveMode::Recursive).unwrap();

//    loop {
//        match rx.recv() {
//            Ok(RawEvent{path: Some(path), op: Ok(WRITE), ..}) => echo_file(path),
//            _ => {}
//        }
//    }
}

fn read_directory(rd: fs::ReadDir) {
//    println!("Value = {:?}", rd);
    for entry in rd {
        match entry {
            Ok(de) => process_entry(de),
            Err(err) => println!("Entry no good: {}", err),
        }
    }
}

fn process_entry(de: fs::DirEntry) {
//    println!("Dir entry = [{:?}]", de);
    let p = de.path();
    let p = p.as_path();
    if p.is_file() {
        process_file(p);
    }
}

fn process_file(p: &Path) {
    println!("File  = [{:?}]", p);
}

//fn echo_file(path_buf: PathBuf) {
//    println!("WRITE to {:?}", path_buf);
//}