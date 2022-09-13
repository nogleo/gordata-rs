

use std::error::Error;
use std::{thread, time::Duration};

use queues::Queue;
use rppal::i2c::I2c;

const ADDR_LSM6DS3: usize = 0x6a;
const ADDR_READ: usize = 0x22;






fn pull_data(duration: Duration) -> Queue<T> {
    let mut q: Queue<T> = Queue::new();
}



fn main() {
    let mut i2c = I2c::new()?;+
    i2c.set_slave_address(ADDR_LSM6DS3)?;

    loop {
        i2c.block_read(REG_READ, &)
    }
    println!("Hello, world!");
}
