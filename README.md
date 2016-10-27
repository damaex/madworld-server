[![Stories in Ready](https://badge.waffle.io/damaex/madworld-server.png?label=ready&title=Ready)](https://waffle.io/damaex/madworld-server)
#MadWorld GameEngine Server

[![Build Status](https://travis-ci.org/damaex/madworld-server.svg?branch=master)](https://travis-ci.org/damaex/madworld-server)

##Building
All dependencies are managed by Cargo, so just:

    cargo build

##Testing
As long as there is no tests for the server, here is a test string for the communication with the websocket server.

TestString:

    {"main":0,"sub":0,"data":"test"}
    
## License
[MIT](./LICENSE.md)