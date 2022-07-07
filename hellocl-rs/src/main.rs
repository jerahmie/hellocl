/*
 * Explore OpenCL with Rust
 */
//use std::io;
//use std::io::Write;
//use std::str;
use opencl3::platform::*;
//use opencl3::error_codes::*;
use cl3::platform::*;
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
    //println!("\tPlatform External Memory Import Handle Types KHR: \n\t\t");
    //let handle_types = p.platform_external_memory_import_handle_types_khr();
    //let handle_types = match handle_types {
    //	Ok(types) => types,
    //	Err(error) => panic!("--> {:?}", error_text(error.0)),
    //};
    //for h in handle_types {
    //	print!("{}, ", h.version);
    //	//io::stdout().flush().unwrap();
    //}
    let platform_name = p.get_data(CL_PLATFORM_NAME).unwrap();
    let platform_name: String = String::from_utf8(platform_name.clone()).unwrap();
    println!("\tCL_PLATFORM_NAME: {}", platform_name.trim());
    let platform_vendor = p.get_data(CL_PLATFORM_VENDOR).unwrap();
    let platform_vendor: String = String::from_utf8(platform_vendor.clone()).unwrap();
    println!("\tCL_PLATFORM_VENDOR: {}", platform_vendor.trim());
    let platform_profile = p.get_data(CL_PLATFORM_PROFILE).unwrap();
    let platform_profile: String = String::from_utf8(platform_profile.clone()).unwrap();
    println!("\tCL_PLATFORM_PROFILE: {}", platform_profile.trim());
    let platform_version = p.get_data(CL_PLATFORM_VERSION).unwrap();
    let platform_version: String = String::from_utf8(platform_version.clone()).unwrap();
    println!("\tCL_PLATFORM_VERSION: {}", platform_version.trim());
    let platform_info: Vec<&str> = platform_version.split_whitespace().collect();
    println!("\tOpenCL version: {}", platform_info[1]);
}

fn device_display(p: &Platform){
    let device_ids = p.get_devices(CL_DEVICE_TYPE_GPU).unwrap();
    println!("\tGPU devices: {:?}", device_ids);
    let device_ids = p.get_devices(CL_DEVICE_TYPE_CPU).unwrap();
    println!("\tCPU devices: {:?}", device_ids);
    let device_info = get_device_info(device_ids[0], CL_DEVICE_TYPE).unwrap();
    let device_info: cl_ulong = cl_ulong::from(device_info); 
    println!("\t{:?}", device_info);
}

fn main() {
    let platforms = get_platforms().unwrap();
    assert!(0 < platforms.len());

    println!("----------------------------------------------------");
    println!("Number OpenCL platforms found: {}", &platforms.len());
    
    for (i, p) in platforms.iter().enumerate() {
	platform_display(i, p);
	println!("\n --- \n");
	device_display(p);
    }
}
