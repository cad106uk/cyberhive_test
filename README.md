# Config #

Create a user called *serve* on the serve machine (87.254.5.232 coding_test_2_2) 
setup *serve* to have passwordless login using ssh. By putting public sshkeys in the authorized)keys file
install rustup and compile the rust code on the server itself

Create a user called *client* on the serve machine (87.254.4.245 coding_test_2_1) 
install python3.8.3 on the client because the version of Python3 on Centos7 is out of date.
setup *client* to have passwordless login using ssh. By putting public sshkeys in the authorized)keys file
Secure connection using ssh from the client machine to the sever machine. 
`ssh -L 7777:127.0.0.1:6969 serve@87.254.5.232 -N -f`
Have the public key on client authorized on the server
Have the client code only use the `localost:7777` address and not connect to the remote server directly

# Codebase #

There are separate codebases here for the server and the client. The server code is in the *server_code* directory and is written in rust version 1.44.1 using the cargo build system

The client code is writen n Python version 3.8.3 and is using virtualenv as its packaging/release system.


# Errors #

The EC2 instances are running centos7, which is just about obsolete. Any rust compiled on a modern system uses GLIBC_2.18 when rust compiled on my laptop throws the following error


`./server_code: /lib64/libc.so.6: version `GLIBC_2.18' not found (required by ./server_code`

My first attempt to fix this was to compile and add the GLIBC_2.18 library the to LD_LIBRARY_PATH. This did not work and just caused everything to throw a segmintation fault.

My second attempt to fix this is to install rustup on the server machine and compile the rust program there. This worked

