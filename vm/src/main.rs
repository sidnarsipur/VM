struct Memory{
    data: [u8; 65536],   
}

impl Memory{
    fn new() -> Self {
        Self { data: [0; 65536] }
    }

    fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}

fn main() {
    let mut mem = Memory::new();

    mem.write(0x1234,42);

    println!("Value is: {}", mem.read(0x1234));
}
