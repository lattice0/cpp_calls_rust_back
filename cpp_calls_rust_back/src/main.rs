use std::{
    io::{Result},
};
use libc::{c_int, c_void, size_t};

struct OVPNClient {
    openvpn_client: *mut c_void,
}

//Just a struct used for implementing the read/write functions
struct OVPNClientInner {
    
}

impl OVPNClientInner {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        println!("read for buf with size {}", buf.len());
        Ok(0)
    }

    fn read_allocate(&mut self, buffer: *mut *mut u8) -> Result<usize> {
        //println!("read for buf with size {}", buf.len());
        println!("OVPNClientInner read_allocate");
        let s:u8 = 10;
        let b: *mut u8 = unsafe{openvpn_client_allocate(s as usize)};
        println!("did allocation");
        for i in 0..s {
            unsafe{
                *b.offset(i as isize) = i;
            }
        }
        println!("finished setting buffer");
        unsafe{
            *buffer = b
        };
        println!("*buffer = b");
        Ok(s as usize)
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        println!("write for buf with size {}", buf.len());
        Ok(0)
    }
}

impl OVPNClient {
    pub fn new() -> OVPNClient {
        let inner = OVPNClientInner{

        };
        let callbacks = Callbacks {
            user_data: Box::into_raw(Box::new(inner)) as *mut c_void,
            on_read: on_read_trampoline,
            on_read_allocate: on_read_allocate_trampoline,
            on_write: on_write_trampoline,
            destroy: destroy_trampoline::<OVPNClientInner>,
        };
        OVPNClient {
            openvpn_client: unsafe{openvpn_client_new(callbacks)},
        }
    }

    pub fn run(&self) -> i32 {
        unsafe{openvpn_client_run(self.openvpn_client)}
    }

}

impl Drop for OVPNClient {
    fn drop(&mut self) {
        unsafe{openvpn_client_free(self.openvpn_client)};
    }
}

fn main() {
    let o = OVPNClient::new();
    let i = o.run();
    println!("run: {}", i);
    std::thread::sleep(std::time::Duration::from_secs(10));
}

extern "C" {
    /// Creates a new OpenVPN C++ client, giving it ownership of the object
    /// inside [`Callbacks`].
    fn openvpn_client_new(callbacks: Callbacks) -> *mut OpenVpnClient;
    /// Tell the OpenVPN client to keep running until the VPN is shut down.
    fn openvpn_client_run(client: *mut OpenVpnClient) -> c_int;
    /// Destroy the OpenVPN client.
    fn openvpn_client_free(client: *mut OpenVpnClient);
    /// Allocates, on C++, a uint8_t* buffer with size `size`
    fn openvpn_client_allocate(size: size_t) -> *mut u8;
}

/// An opaque type representing the C++ OpenVPN client.
type OpenVpnClient = c_void;

#[repr(C)]
pub struct Callbacks {
    /// A pointer to some user-defined state.
    pub user_data: *mut c_void,
    /// Callback fired when the OpenVPN client wants to read data.
    pub on_read: unsafe extern "C" fn(*mut u8, size_t, *mut c_void) -> c_int,
    /// Callback fired when the OpenVPN client wants to read data but does not know the data size
    /// so it leaves to Rust the task of allocating. Returns 0 on success, -1 on failure
    pub on_read_allocate: unsafe extern "C" fn(*mut *mut u8, *mut size_t, *mut c_void) -> c_int,
    /// Callback fired when the OpenVPN client wants to write some data.
    pub on_write: unsafe extern "C" fn(*const u8, size_t, *mut c_void) -> c_int,
    /// A function for destroying the user-defined state.
    pub destroy: unsafe extern "C" fn(*mut c_void),
}

unsafe extern "C" fn on_read_trampoline(
    buffer: *mut u8,
    len: size_t,
    user_data: *mut c_void,
) -> c_int {
    let user_data = &mut *(user_data as *mut OVPNClientInner);
    let buffer = std::slice::from_raw_parts_mut(buffer as *mut u8, len as usize);

    match user_data.read(buffer) {
        Ok(bytes_read) => bytes_read as c_int,
        Err(_) => -1,
    }
}

unsafe extern "C" fn on_read_allocate_trampoline(
    buffer: *mut *mut u8,
    len: *mut size_t,
    user_data: *mut c_void,
) -> c_int {
    println!("on_read_allocate_trampoline");
    let user_data = &mut *(user_data as *mut OVPNClientInner);
    //let buffer = std::slice::from_raw_parts_mut(buffer as *mut u8, len as usize);
    match user_data.read_allocate(buffer) {
        Ok(allocated_size) => {
            println!("Ok(allocated_size) for allocated_size: {}", allocated_size);
            *len = allocated_size;
            println!("returning from on_read_allocate_trampoline");
            0 as c_int
        },
        Err(_) => {
            println!("err from read_allocate");
            -1
        },
    }
}

unsafe extern "C" fn on_write_trampoline(
    buffer: *const u8,
    len: size_t,
    user_data: *mut c_void,
) -> c_int {
    let user_data = &mut *(user_data as *mut OVPNClientInner);
    let buffer = std::slice::from_raw_parts(buffer as *const u8, len as usize);

    match user_data.write(buffer) {
        Ok(bytes_written) => bytes_written as c_int,
        Err(_) => -1,
    }
}

unsafe extern "C" fn destroy_trampoline<P>(user_data: *mut c_void) {
    let user_data = Box::from_raw(user_data as *mut P);
    drop(user_data);
}