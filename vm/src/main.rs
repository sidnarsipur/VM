// Physical Memory: 2^16 Bytes
// Virtual Memory: 2^32 Bytes
// Page Size: 2^11 Bytes (2KB)

//No of virtual pages = 2^32 / 2^11 = 2^21
//No of physical frames = 2^16 / 2^11 = 2^5 = 32

//Each virtual address contains 32 bits: 21 bits (page number) and 11 bits (offset)
//Each page table entry contains 6 bits: 1 bit (valid bit) and 5 bits (frame number)

//Each page table has 2^21 page table entries.

struct Memory {
    data: [u8; 65536], //16-bit physical page no that stores one byte
}

impl Memory {
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
  
struct PageTable {
    data: [u8; 2097152] //21-bit virtual page no; Lower 6 bits are used as PTE.
}

impl PageTable {
    fn new() -> Self {
        //Default PTE is 0 for valid bit and 0000 for frame number
        Self { data: [0; 2097152]}
    }

    fn is_valid(&self, virtual_address: u32) -> bool {
        //Want upper 21 bits from the virtual address
        // 1111 1111 1111 1111 1111 1000 0000 0000
        // F    F    F    F    F    8    0    0
        let page_number: u32 = (virtual_address & 0xFFFFF800) >> 11;
        let pt_entry: u8 = self.data[page_number as usize];
        
        //Valid bit is 3rd bit
        //0010 0000
        //2     0
        let valid_bit: u8 = (pt_entry & 0x20) >> 5;
        return valid_bit == 1
    }

    //TODO: move to an MMU impl?
    fn get_physical_address(&self, virtual_address: u32) -> u16 {
        if !self.is_valid(virtual_address) {
            //TODO: handle page fault
            return 0;
        }

        let page_number: u32 = (virtual_address & 0xFFFFF800) >> 11;
        //Want upper 11 bits from the virtual address
        // 0000 0000 0000 0000 0000 0111 1111 1111
        // 0    0    0    0    0    E    F    F
        let offset: u32 = virtual_address & 0x00000EFF;

        let pt_entry: u8 = self.data[page_number as usize];
        //Want lower 5 bits
        //0001 0000
        //1    0
        let frame_number = (pt_entry & 0x10) as u32;

        //Combine 5 bits of frame number and 11 bits of offset to derive 16 bit physical address no.
        let physical_address = ((frame_number << 11) | offset) as u16;

        return physical_address
    }
}

fn main() {
    let mut mem = Memory::new();

    mem.write(0x1234,42);

    println!("Value is: {}", mem.read(0x1234));
}
