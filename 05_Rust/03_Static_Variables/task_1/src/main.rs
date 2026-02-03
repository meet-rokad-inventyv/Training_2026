mod rwlock;
mod mutex;
fn main() {
    println!();
    println!("----Use Mutex----");
    mutex::mutex();

    println!();
    println!("-----Use RwLock-----");
    rwlock::rwlock();
}

