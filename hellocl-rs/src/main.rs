/*
 * Explore OpenCL with Rust
 */
use std::io;
use std::io::Write;
use opencl3::platform::*;
use opencl3::error_codes::*;
use cl3::device::*;

fn platform_display(id: usize, p: &Platform) ->() {
    println!("     -------------- OpenCL Device Summary -------------------");
    println!("\tPlatform Number: {}", id);
    println!("\tPlatform Name: {}", p.name().unwrap());
    println!("\tPlatform Profile: {}", p.profile().unwrap());
    println!("\tPlatform Version: {}", p.version().unwrap());
    println!("\tPlatform Vendor: {}", p.vendor().unwrap());
    println!("\tPlatform Extensions: {}", p.extensions().unwrap());
    //println!("\tPlatform Host Timer Resulution: {}", p.host_timer_resolution().unwrap());
    //println!("\tPlatform Numeric Version: {}", p.numeric_version().unwrap());
    //println!("\tPlatform Extensions with Version: {}", p.extensions_with_version().unwrap());
    println!("\tPlatform External Memory Import Handle Types KHR: \n\t\t");
    
    let handle_types = p.platform_external_memory_import_handle_types_khr();
    let handle_types = match handle_types {
	Ok(types) => types,
	Err(error) => panic!("--> {:?}", error_text(error.0)),
    };
    for h in handle_types {
	print!("{}, ", h.version);
	//io::stdout().flush().unwrap();
    }
    
}

fn main() {
    let platforms = get_platforms().unwrap();
    assert!(0 < platforms.len());
    
    println!("Number OpenCL platforms found: {}", &platforms.len());
    for (i, p) in platforms.iter().enumerate() {
	platform_display(i, p);
    }
}
