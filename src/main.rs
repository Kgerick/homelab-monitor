use sysinfo::{Disks, System};
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

    let disks = Disks::new_with_refreshed_list();

    println!("\nStorage");
    println!("-------");

    for disk in disks.list() {
        let total_gb = disk.total_space() / 1_000_000_000;
        let available_gb = disk.available_space() / 1_000_000_000;
        let used_gb = total_gb - available_gb;

        println!(
            "{}: {} GB used / {} GB total", disk.mount_point().display(), used_gb, total_gb
        );
    }
}
