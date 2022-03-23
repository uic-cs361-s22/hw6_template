use std::io::Write;
pub fn print_chunk(out: &mut dyn Write, ptr: *const u8) {
    // make sure to use write!, not format! here, since format! 
    // allocates a String, thus modifying the heap for you!
 }
 
 pub fn print_heap(out: &mut dyn Write, from: *const u8) {
    // make sure to use write!, not format! here, since format! 
    // allocates a String, thus modifying the heap for you!
    
 }
 
 pub fn print_freelist(out: &mut dyn Write, hdr: *const u8) {
    // make sure to use write!, not format! here, since format! 
    // allocates a String, thus modifying the heap for you!
     
 }
 