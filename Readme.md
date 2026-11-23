# Vrot Experiments

The following repo contains an example for the vrot framework.
To run the example we need to have a copy of the vrot framework which has to be placed in
the root of the project with the name vrot (a symlink is also valid).

The following example represents a system composed of two boards which communicate through
a serial (UART) communication.
Each board has a virtual peripheral (p2p_uart) which is responsible for the communication.
Then a Virtual Bus (p2p_bus) is created to link the two peripherals.
Also an rtc timer has been added to the server and it will dictate when the server will send the
data.

The purpose of the system is to send the answer to the universe from the oracle (server) to
the humanity (client).

# Requirements

- vrot
- [make](https://www.gnu.org/software/make/)
- [Rust](https://rust-lang.org/it/)
- [amd64 architecture](https://it.wikipedia.org/wiki/AMD64)
- [Linux](https://it.wikipedia.org/wiki/Linux)
- internet connection (maybe)

# Setup

Before running the example you need to configure the working environment. To do that
run this command from the *project's root*:
```sh
./setup.sh
```
> [!IMPORTANT]
> This script requires:
> - [curl](https://github.com/curl/curl)
> - [cmake](https://cmake.org/)
> - [tar](https://www.gnu.org/software/tar/)
> - [git](https://git-scm.com/)

# Running

To run the example, assuming all the Requirements have been satisfied, run the following command:
```sh
make run
```

The output of the stdio can be ignored unless you want to dive in the implementation.
Two files will be generated:

- cluster_server.log
- cluster_client.log

These two files represents the log of the boards and the expected output are the following:

cluster_server.log:
```txt
server:waiting interrupt
server:waiting interrupt
server:triggered send interrupt
server:sending data: 42
server:data sent: 42
server:triggered send interrupt
server:sending data: 42
server:data sent: 42
server:triggered send interrupt
server:triggered send interrupt
server:triggered send interrupt
```

cluster_client.log:
```txt
client:starting loop
client:waiting message
client:waiting message
client:waiting message
client:recv interrupt triggered
client:received msg
client:uart read: 42
client:recv payload: 42
```

If you want to only build the Packages run:
```sh
make build
```

To clean the environment run:
```sh
make clean
```
