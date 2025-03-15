use shared_memory::{Shmem, ShmemConf};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct SharedCounter {
    shmem: Shmem,
}

impl SharedCounter {
    pub fn create(name: &str) -> Self {
        let shmem = ShmemConf::new()
            .os_id(name)
            .size(std::mem::size_of::<AtomicU64>())
            .create()
            .expect("Failed to create shared memory");

        // Initialize the counter to 0
        let counter = unsafe { &mut *(shmem.as_ptr() as *mut AtomicU64) };
        *counter = AtomicU64::new(0);

        Self { shmem }
    }

    pub fn open(name: &str) -> Self {
        let shmem = ShmemConf::new()
            .os_id(name)
            .open()
            .expect("Failed to open shared memory");

        Self { shmem }
    }

    pub fn increment(&self) -> u64 {
        let counter = unsafe { &*(self.shmem.as_ptr() as *const AtomicU64) };
        counter.fetch_add(1, Ordering::SeqCst) + 1
    }

    pub fn get(&self) -> u64 {
        let counter = unsafe { &*(self.shmem.as_ptr() as *const AtomicU64) };
        counter.load(Ordering::SeqCst)
    }
}
