use std::{
    io::{Read, Write, Result},
    os::raw::{c_char, c_int, c_void},
};

//use std::sync::{Arc, Mutex};

struct OVPNClient {
    openvpn_client: *mut c_void,
}

struct OVPNClientInner {
    
}

impl Read for OVPNClientInner {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        println!("read for buf with size {}", buf.len());
        Ok(0)
    }
}

impl Write for OVPNClientInner {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        println!("write for buf with size {}", buf.len());
        Ok(0)
    }
    fn flush(&mut self) -> Result<()> {
        println!("flush!");
        Ok(())
    }
}

impl OVPNClient {
    pub fn new() -> OVPNClient {
        let inner = OVPNClientInner{

        };
        let callbacks = Callbacks {
            user_data: Box::into_raw(Box::new(inner)) as *mut c_void,
            on_read: on_read_trampoline::<OVPNClientInner>,
            on_write: on_write_trampoline::<OVPNClientInner>,
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
}

/// An opaque type representing the C++ OpenVPN client.
type OpenVpnClient = c_void;

#[repr(C)]
pub struct Callbacks {
    /// A pointer to some user-defined state.
    pub user_data: *mut c_void,
    /// Callback fired when the OpenVPN client wants to read data.
    pub on_read: unsafe extern "C" fn(*mut c_char, c_int, *mut c_void) -> c_int,
    /// Callback fired when the OpenVPN client wants to write some data.
    pub on_write: unsafe extern "C" fn(*const c_char, c_int, *mut c_void) -> c_int,
    /// A function for destroying the user-defined state.
    pub destroy: unsafe extern "C" fn(*mut c_void),
}

unsafe extern "C" fn on_read_trampoline<P: Read>(
    buffer: *mut c_char,
    len: c_int,
    user_data: *mut c_void,
) -> c_int {
    let user_data = &mut *(user_data as *mut P);
    let buffer = std::slice::from_raw_parts_mut(buffer as *mut u8, len as usize);

    match user_data.read(buffer) {
        Ok(bytes_read) => bytes_read as c_int,
        Err(_) => -1,
    }
}

unsafe extern "C" fn on_write_trampoline<P: Write>(
    buffer: *const c_char,
    len: c_int,
    user_data: *mut c_void,
) -> c_int {
    let user_data = &mut *(user_data as *mut P);
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
