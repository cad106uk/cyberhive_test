# Execution #

How to run this system

### Server Side ###

  * clone this git repo
  * go to the server_code directory
  * run `cargo run --release &`
  * run `tail -f log_records.json`

### Client Side ###

  * clone this git repo
  * go to the client_code/bin directory
  * run `source run.sh`

# Config #

### Server Config ###
  * Create a user called *serve* on the serve machine (87.254.5.232 coding_test_2_2)
  * setup *serve* to have password-less login using SSH.
  * install rustup and compile the rust code on the server itself, because Centos7 version of glibc is out of date

### Client Config ###

  * Create a user called *client* on the serve machine (87.254.4.245 coding_test_2_1)
  * install python3.8.3 on the client because the version of Python3 on Centos7 is out of date.
  * setup *client* to have password-less login using SSH. By putting public sshkey in the authorized_keys file on the server
  * Secure connection using SSH from the client machine to the sever machine.
  * `ssh -L 7777:127.0.0.1:6969 serve@87.254.5.232 -N -f`
  * Have the public key on client authorized on the server
  * Have the client code only use the `localost:7777` address and not connect to the remote server directly

# Code base #

There are separate code bases in this one repo for the server and the client. The server code is in the *server_code* directory and is written in rust version 1.44.1 using the cargo build system

The client code is written n Python version 3.8.3 and is using virtualenv as its packaging/release system.

# Design Notes #

I am using SSH iptunnelling to handle the communication, login and security between the various machines. The server and client process only talk to localhost. I wanted to keep my code base as simple as possible and make sure I handled a basic level of security

I have used 2 language in my solution Rust on the server side and Python on the client side. I think this represents the real world, server side software being as optimized as possible and the client side using higher level approach.

I have JSON to encode the data being sent. This is a simple flexible format so each client can choose its own data to send. We are not just accepting random text. The server side can then validate that at least correctly formatted data has been sent.

The server records each message by appending a line for json formatted string to the log file.

The server side code is written in tokio to allow non-blocking code using co-routines to allow multiple clients to access the server concurrently. The calls are handled through a queue to make sure each record to the log is written sequentiality

The client uses the command `ps aux` to get a list of all the currently running process. The data from this command is transformed from a string buffer into a list of dictionaries. This list of dictionaries is turn into JSON and sent to the server.

# Testing #

Current I have only done manual testing, because this project is so small.

To test the server side, I could write unit tests for the RecordJson traits implementation. The only real way to test something this small is with integration tests. I would have to start the server in a prepared test environment, pass in a prepared fixture of data, then check that the correct results have bee produced.

For the client side I would rely mostly on unit tests. By mocking out the TCP connection I can write unit tests to show that all client functionality is working.

# Future Development #

Remove the hard coding of ipaddresses and port numbers.

More structure in the log records being recorded. Perhaps also add the timestamps of when this was received and which client it was received from.

The server should return meaningful error messages to the client system. Right now there are no success or error messages sent from the server to the client.

# Errors #

The EC2 instances are running centos7, which is just about obsolete. Any rust compiled on a modern system uses GLIBC_2.18 when rust compiled on my laptop throws the following error


`./server_code: /lib64/libc.so.6: version `GLIBC_2.18' not found (required by ./server_code`

My first attempt to fix this was to compile and add the GLIBC_2.18 library the to LD_LIBRARY_PATH. This did not work and just caused everything to throw a segmentation fault.

My second attempt to fix this is to install rustup on the server machine and compile the rust program there. This worked

Forgot to flush and cleant he sockets for the client end. Now fixed
