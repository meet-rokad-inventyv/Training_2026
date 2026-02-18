use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
struct MultiThread {
    id: i32,
    record_added_time: Instant,
    record_added_time_str: String,
    thread_id: String,
}

static GLOBAL_ID: AtomicI32 = AtomicI32::new(1);

fn generate_thread_id() -> String {
    let mut rng = thread_rng();
    format!("T-{}", rng.gen_range(1000..9999))
}

fn main() {
    let records: Arc<Mutex<Vec<MultiThread>>> = Arc::new(Mutex::new(Vec::new()));

    /* --------------------------------------------------
       Thread 1 — Record Creator (every 10 seconds)
    -------------------------------------------------- */
    {
        let records = Arc::clone(&records);
        thread::spawn(move || loop {
            let id = GLOBAL_ID.fetch_add(1, Ordering::SeqCst);
            let now = Instant::now();

            let record = MultiThread {
                id,
                record_added_time: now,
                record_added_time_str: format!("{:?}", now),
                thread_id: generate_thread_id(),
            };

            records.lock().unwrap().push(record);
            println!("Record Created");

            thread::sleep(Duration::from_secs(10));
        });
    }

    /* --------------------------------------------------
       Thread 2 — State Printer
    -------------------------------------------------- */
    {
        let records = Arc::clone(&records);
        thread::spawn(move || loop {
            let data = records.lock().unwrap();
            println!("\nCurrent Records ({}):", data.len());
            for r in data.iter() {
                println!("{:?}", r);
            }
            drop(data);
            thread::sleep(Duration::from_secs(5));
        });
    }

    /* --------------------------------------------------
       Thread 3 — Even Record Cleaner
    -------------------------------------------------- */
    {
        let records = Arc::clone(&records);
        thread::spawn(move || loop {
            let mut data = records.lock().unwrap();
            let now = Instant::now();

            data.retain(|r| {
                !(r.id % 2 == 0 && now.duration_since(r.record_added_time).as_secs() > 20)
            });

            thread::sleep(Duration::from_secs(5));
        });
    }

    /* --------------------------------------------------
       Thread 4 — Odd Record Cleaner
    -------------------------------------------------- */
    {
        let records = Arc::clone(&records);
        thread::spawn(move || loop {
            let mut data = records.lock().unwrap();
            let now = Instant::now();

            data.retain(|r| {
                !(r.id % 2 != 0 && now.duration_since(r.record_added_time).as_secs() > 20)
            });

            thread::sleep(Duration::from_secs(5));
        });
    }

    /* --------------------------------------------------
       Thread 5 — Even Counter
    -------------------------------------------------- */
    {
        let records = Arc::clone(&records);
        thread::spawn(move || loop {
            let data = records.lock().unwrap();
            let count = data.iter().filter(|r| r.id % 2 == 0).count();
            println!("🔢 Even Records Count: {}", count);
            drop(data);
            thread::sleep(Duration::from_secs(5));
        });
    }

    /* --------------------------------------------------
       Thread 6 — Odd Counter
    -------------------------------------------------- */
    {
        let records = Arc::clone(&records);
        thread::spawn(move || loop {
            let data = records.lock().unwrap();
            let count = data.iter().filter(|r| r.id % 2 != 0).count();
            println!("🔢 Odd Records Count: {}", count);
            drop(data);
            thread::sleep(Duration::from_secs(5));
        });
    }

    // Keep main thread alive
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
