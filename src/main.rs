use libc;
use std::str;
use std::mem::size_of;

fn main() {
    let listening_port = 8080 as u16;

    println!("listening port is {}", listening_port);

    let socket_addr = libc::sockaddr_in{
        sin_family: libc::AF_INET as u16,
        sin_port: listening_port.swap_bytes(),
        sin_addr: libc::in_addr{
            s_addr: libc::INADDR_ANY
        },
        sin_zero: [0, 0, 0, 0, 0, 0, 0, 0]
    };

    let mut socket_addr_peer = libc::sockaddr_in{
        sin_family: libc::AF_INET as u16,
        sin_port: listening_port.swap_bytes(),
        sin_addr: libc::in_addr{
            s_addr: libc::INADDR_ANY
        },
        sin_zero: [0, 0, 0, 0, 0, 0, 0, 0]
    };

    let socket_addr_peer_ptr = &mut socket_addr_peer as *mut libc::sockaddr_in as *mut libc::sockaddr;

    let mut buffer : [u8; 1024] = [0; 1024];
    let buffer_ptr = &mut buffer as *mut [u8; 1024] as *mut libc::c_void;

    let mut socket_addr_size = size_of::<libc::socklen_t>() as u32;

    let file_descriptor: i32;

    unsafe {
        // First I need to create a socket that can be used.
        file_descriptor = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
    }

    let mut socket_opt = 1;
    let socket_opt_ptr = &mut socket_opt as *mut i32 as *mut libc::c_void;

    let set_socket_result: i32;

    unsafe {
        set_socket_result = libc::setsockopt(file_descriptor,
            libc::SOL_SOCKET,
            libc::SO_REUSEADDR | libc::SO_REUSEPORT,
            socket_opt_ptr,
            size_of::<i32>() as u32);
    }

    println!("socket set result {}", set_socket_result);

    let socket_addr_ptr = &socket_addr as *const libc::sockaddr_in as *const libc::sockaddr;

    let bind_result: i32;

    unsafe {
        bind_result = libc::bind(file_descriptor, socket_addr_ptr, size_of::<libc::sockaddr_in>() as u32);
    }

    println!("bind result {}", bind_result);

    let listening_result: i32;

    unsafe {
        listening_result = libc::listen(file_descriptor, 3);

        println!("listening result {}", listening_result);
    }

    let new_socket: i32;

    unsafe {
        new_socket = libc::accept(file_descriptor, socket_addr_peer_ptr, &mut socket_addr_size);
    }

    println!("accepted new connection {}", new_socket);

    let amount_read: isize;

    unsafe {
        amount_read = libc::read(new_socket, buffer_ptr, 1024);
    }

    println!("bytes read {} on port {}", amount_read, socket_addr_peer.sin_port);

    unsafe {
        println!("errno {}", *libc::__errno_location());
    }

    let result = String::from(str::from_utf8(&buffer).unwrap_or_default());
    
    println!("read the following from the client -\n{}", result);
}
