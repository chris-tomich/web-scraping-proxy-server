# What do I need to do?
* Need to listen to the port.
    * There's a skeleton of what needs to be done here
        - https://www.geeksforgeeks.org/socket-programming-cc/
* When I needed to figure out the size of a struct and casting the following references helped
    - https://stackoverflow.com/questions/28273169/how-do-i-convert-between-numeric-types-safely-and-idiomatically
    - https://doc.rust-lang.org/std/mem/fn.size_of.html
* Need to fill in sockadd_in. Had to use the following references
    - https://www.ict.griffith.edu.au/teaching/7421ICT/archive/guide/bgnet/output/html/sockaddr_inman.html
    - https://man7.org/linux/man-pages/man2/socket.2.html
    - https://man7.org/linux/man-pages/man2/getsockopt.2.html
    - https://man7.org/linux/man-pages/man3/setsockopt.3p.html
    - https://docs.rs/libc/0.2.74/libc/struct.sockaddr_in.html
    - https://docs.rs/libc/0.2.74/libc/struct.in_addr.html
    - https://users.rust-lang.org/t/help-understanding-libc-call/17308/9
    * Note that the port needs to be in network-order (big endian);
* For working out bind I was able to use some of the above plus the following
    - https://docs.rs/libc/0.2.74/libc/fn.bind.html
    - https://man7.org/linux/man-pages/man2/bind.2.html
    * Just a note, it turns out when you're using bind for setting up a local port, you don't need to specify an address in sockadd_in. I just ended up trying 0 which is the same as the constant INADDR_ANY.
    * I don't think you can listen on the loopback interface. I had trouble when I attempted to do it.
* When I needed details on the error number returned from a failed call I used the following references
    - https://docs.rs/libc/0.2.74/libc/fn.__errno_location.html
        * So there was some learning about this done reading [this](https://www.reddit.com/r/rust/comments/bz6o1l/how_can_i_check_the_value_of_errno_without_libc/)
        * Summary is, errno is a really sucky way of returning the last error in linux because C doesn't have multiple return values. As a result, the error doesn't necessarily exist until an error occurs so it may not exist.
    - https://www-numi.fnal.gov/offline_software/srt_public_context/WebDocs/Errors/unix_system_errors.html
* Used the following for listening to the socket.
    - https://man7.org/linux/man-pages/man2/listen.2.html
* Used the following for accepting an incoming connection.
    - https://man7.org/linux/man-pages/man2/accept.2.html
* When reading the bytes and string I used the following references.
    - https://doc.rust-lang.org/std/str/fn.from_utf8.html