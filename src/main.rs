use std::thread;
use std::time::Duration;
use std::thread::sleep;
use std::sync::mpsc;

use rand::Rng;

const FULL_TANK: i32 = 5000;
const MAX_CONSUMPTION: i32 = 300;

fn main() {
    let mut rocket_fuel_tank = FULL_TANK;

    // Set up inter-thread channels
    let (rocket_fuel_req_tx, rocket_fuel_req_rx) = mpsc::channel();
    let (rocket_fuel_data_tx, rocket_fuel_data_rx) = mpsc::channel();

    // Spawn a thread to simulate fuel consumption
    let handle = thread::spawn(move || {
        loop {
            let consumption = rand::thread_rng().gen_range(1..MAX_CONSUMPTION);
            let starting = rocket_fuel_tank;

            rocket_fuel_tank -= consumption;
            if rocket_fuel_tank <= 0 { rocket_fuel_tank = 0; }

            println!("spawned thread: {starting} - {consumption} = {rocket_fuel_tank} remaining");

            // Respond to main thread’s request for fuel
            if rocket_fuel_req_rx.try_recv().is_ok() {
                let _ = rocket_fuel_data_tx.send(rocket_fuel_tank);
            }

            if rocket_fuel_tank == 0 {
                println!("spawned thread ended.");
                break;
            } else {
                sleep(Duration::from_millis(100));
            }
        }
    });

    // Poll the remaining volume until the tank is empty
    loop {
        // Simulate a poll every half second
        sleep(Duration::from_millis(500));
        // Request data from the spawned thread
        if rocket_fuel_req_tx.send(()).is_ok() {
            match rocket_fuel_data_rx.recv_timeout(Duration::from_millis(100)) {
                Ok(data) => {
                    println!("Rocket fuel remaining: {}", data);
                    if data == 0 {
                        break;
                    }
                }
                Err(_) => {
                    println!("No response from spawned thread. Assuming 0.");
                    break;
                }
            }
        } else {
            println!("Could not request data from spawned thread. Assuming 0.");
            break;
        }
    }

    handle.join().unwrap();
}
