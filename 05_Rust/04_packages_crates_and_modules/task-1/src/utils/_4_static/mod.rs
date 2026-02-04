mod rwlock;
mod mutex;
pub fn main() {
    println!();
    println!("----Use Mutex----");
    mutex::mutex();

    println!();
    println!("-----Use RwLock-----");
    rwlock::rwlock();
}

