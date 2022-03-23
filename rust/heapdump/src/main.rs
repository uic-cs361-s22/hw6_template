use libc;
use std::io::Write;

mod heapdump;
use heapdump::*;

unsafe fn find_freelist(size: usize) -> *const u8 {
    let ptr = libc::malloc(size);
    let ptr2 = libc::malloc(size);
    libc::free(ptr);
    let hdr = *(ptr as *const usize)+16;
    libc::free(ptr2);
    return hdr as _;
}

fn main() {
    println!("Allocating a chunk to serve as the bottom of our heap.\n");
    let mut stdout = std::io::stdout();
    unsafe {
        let bottom = Box::new([0u8;1024]);
        let bottom_ptr = Box::into_raw(bottom);
        print_chunk(&mut stdout,bottom_ptr as _);
        println!("\nCalling malloc_trim(0)");
        libc::malloc_trim(0);
        print_chunk(&mut stdout,bottom_ptr as _);

        println!("\nFull heap is");
        print_heap(&mut stdout,bottom_ptr as _);
        
        println!("\nMallocing some chunks");
        print_heap(&mut stdout,bottom_ptr as _);
        const ARR200: Option<Box<[u8;200]>> = None;
        let mut chunks=[ARR200;10];
        libc::malloc_trim(0);
        print_heap(&mut stdout,bottom_ptr as _);

        for i in 0..5 {
            chunks[i]=Some(Box::new([0u8;200]));
            print_heap(&mut stdout,bottom_ptr as _);
        }

        println!("\nNow freeing them in reverse order");
        for i in (0..5).rev() {
            chunks[i]=None;
            print_heap(&mut stdout,bottom_ptr as _);
        }

        println!("\nMallocing some chunks again");
        for i in 0..5 {
            chunks[i]=Some(Box::new([0u8;200]));
            print_heap(&mut stdout,bottom_ptr as _);
        }

        println!("\nbut now freeing them in the same order. Notice the difference.");
        for i in 0..5 {
            chunks[i]=None;
            print_heap(&mut stdout,bottom_ptr as _);
        }

        for i in 0..10 {
            chunks[i]=Some(Box::new([0xffu8;200]));
        }

        println!("\nBuilding a freelist of 200 byte chunks");

        let hdr = find_freelist(200);
        for i in (0..10).step_by(2) {
            chunks[i]=None;
            print_freelist(&mut stdout,hdr);
        }

        println!("Freeing the rest");
        for i in (1..10).step_by(2) {
            chunks[i]=None;
            print_freelist(&mut stdout,hdr);
        }

        println!("Building another freelist of 200 byte chunks");
        for i in 0..10 {
            chunks[i]=Some(Box::new([0xffu8;200]));
        }
        for i in (0..10).step_by(2) {
            chunks[i]=None;
            print_freelist(&mut stdout,hdr);
        }
        
        println!("\nBut one single malloc, and the freelist looks like this:");
        let b = Box::new([0u8;1000]);
        print_freelist(&mut stdout,hdr);
        println!("\nIt's empty! Can you explain why? (hint: have a closer look at what happened when 'freeing the rest' above)");
    }
}
