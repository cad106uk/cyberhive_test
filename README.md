# Config #

Create a user called *serve* on the serve machine (87.254.5.232 coding_test_2_2) 
setup *serve* to have passwordless login using ssh. By putting public sshkeys in the authorized)keys file

Create a user called *client* on the serve machine (87.254.4.245 coding_test_2_1) 
setup *serve* to have passwordless login using ssh. By putting public sshkeys in the authorized)keys file

# Codebase #

There are separate codebases here for the server and the client. The server code is in the *server_code* directory and is written in rust version 1.44.1 using the cargo build system

The client code is writen n Python version 3.8.3 and is using virtualenv as its packaging/release system.
