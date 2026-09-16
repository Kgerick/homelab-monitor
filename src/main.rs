use sysinfo::System;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Homelab Monitor");
    println!("===============");

    let mut system = System::new_all();

    system.refresh_all();

    thread::sleep(Duration::from_millis(500));
    system.refresh_cpu_usage();

    println!("Total memory:  {} MB", system.total_memory() / 1024 / 1024);
    println!("Used memory:   {} MB", system.used_memory() / 1024 / 1024);
    println!("CPU usage:     {:.1}%", system.global_cpu_usage());
}
