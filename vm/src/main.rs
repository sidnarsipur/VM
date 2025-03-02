// Physical Memory: 2^16 Bytes
// Virtual Memory: 2^32 Bytes
// Page Size: 2^11 Bytes (2KB)

//No of virtual pages = 2^32 / 2^11 = 2^21
//No of physical frames = 2^16 / 2^11 = 2^5 = 32

//Each virtual address contains 32 bits: 21 bits (page number) and 11 bits (offset)
//Each page table entry contains 8 bits: 1 modified bit, 1 dirty bit, 1 valid bit, and 5 bits for the frame number

//Each page table has 2^21 page table entries.

use core::error;
use std::collections::LinkedList;

struct Memory {
    data: [u8; 65536], //16-bit byte-addressable main memory
    frame_table: [u32; 32]
}

impl Memory {
    const fn new() -> Self {
        Self { data: [0; 65536], frame_table: [0; 32] }
    }

    fn load(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn store(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}

static mut MAIN_MEMORY: Memory = Memory::new();
  
struct PageTable {
    data: [u8; 2097152], //21-bit virtual memory
    lru_stack: LinkedList<u32>,
}

impl PageTable {
    fn new() -> Self {
        //Default PTE is 0 for valid bit and 0000 for frame number
        Self { data: [0; 2097152], lru_stack: LinkedList::new() }
    }

    fn is_valid(&self, virtual_address: u32) -> bool {
        //Want upper 21 bits from the virtual address
        // 1111 1111 1111 1111 1111 1000 0000 0000
        // F    F    F    F    F    8    0    0
        let virtual_page_number: u32 = (virtual_address & 0xFFFFF800) >> 11;
        let pt_entry: u8 = self.data[virtual_page_number as usize];
        
        //Valid bit is 3rd bit
        //0010 0000
        //2     0
        let valid_bit: u8 = (pt_entry & 0x20) >> 5;
        return valid_bit == 1
    }
}

fn get_physical_address(page_table: &mut PageTable, virtual_address: u32) -> u16 {
    if !page_table.is_valid(virtual_address) {
        //TODO: handle page fault
        return 0;
    }

    let virtual_page_number: u32 = (virtual_address & 0xFFFFF800) >> 11;
    //Want upper 11 bits from the virtual address
    // 0000 0000 0000 0000 0000 0111 1111 1111
    // 0    0    0    0    0    E    F    F
    let offset: u32 = virtual_address & 0x00000EFF;

    page_table.lru_stack.push_back(virtual_page_number);

    let pt_entry: u8 = page_table.data[virtual_page_number as usize];
    //Want lower 5 bits
    //0001 0000
    //1    0
    let frame_number = (pt_entry & 0x10) as u32;

    //Combine 5 bits of frame number and 11 bits of offset to derive 16 bit physical address no.
    let physical_address = ((frame_number << 11) | offset) as u16;

    return physical_address
}

fn handle_page_fault(page_table: &mut PageTable, virtual_address: u32) {
    let evicted_virtual_page_number: Option<&u32> = page_table.lru_stack.front();

    //TODO:
    //Check if main memory is full, and evict only if it is. Otherwise, just assign the frame to the page.

    if let Some(&page_number) = evicted_virtual_page_number {
        let evicted_pt_entry: u8 = page_table.data[page_number as usize];

        //Set valid bit to 0
        //1101 1111
        //D      F 
        let new_evicted_pt_entry: u8 = evicted_pt_entry & 0xDF;
        page_table.data[page_number as usize] = new_evicted_pt_entry;

        let frame_number = evicted_pt_entry & 0x10;
        let virtual_page_number: u32 = (virtual_address & 0xFFFFF800) >> 11;

        //Modified bit is 0, dirty bit is 0, valid bit is 1, frame number
        //0011 1111
        //   3  F
        let new_pt_entry = frame_number & 0x3F;

        page_table.data[virtual_page_number as usize] = new_pt_entry;

        return
    }

}

//Assign a frame to a virtual page
fn assign_page(page_table: &mut PageTable, virtual_page_number: u32, frame_number: u8) {
    let valid_bit: u8 = 1;
    let dirty_bit: u8 = 0;

    let pt_entry = (valid_bit << 5) | (dirty_bit << 6) | (frame_number);

    page_table.data[virtual_page_number as usize] = pt_entry;
}

fn main() {
    let mut mem = Memory::new();

    mem.store(0x1234,42);

    println!("Value is: {}", mem.load(0x1234));
}
