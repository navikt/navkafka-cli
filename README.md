# navkafka-cli
[![Build Status](https://travis-ci.org/navikt/navkafka-cli.svg?branch=master)](https://travis-ci.org/navikt/navkafka-cli)

Application to simplify running the local instance of a NAV prod-alike kafka cluster using the [docker-compose 
project](https://github.com/navikt/navkafka-docker-compose).

### Features
* Add users to the ldap instance

### Technologies & Tools

* Rust
* ldap3
* clap

### Getting started
# Build requirements
 To build you need a functional rust build environment, the easiest way of getting this is by installing rustup
# Build the application
To build the application just run ```cargo build``` or for a release build ```cargo build --release```.

You can also run the application right after building by doing ```cargo run <args>```


### Contact us
#### Code/project related questions can be sent to 
* Kevin Sillerud, `kevin.sillerud@nav.no`

#### For NAV employees
We are also available on the slack channel #integrasjon for internal communication.
