# ebc-hub

Remote control and monitoring hub for ZKETECH EBC battery testers.

`ebc-hub` is a Rust-based project for controlling and monitoring ZKETECH EBC battery testers from a Raspberry Pi, server, or desktop machine.

The long-term goal is to provide a central hub with:

- Multi-device support
- Web-based user interface
- Remote monitoring
- Data logging
- Battery test automation

## Current Status

The project is currently in an early development stage.

Implemented:

- Device discovery via configuration
- Connect / Disconnect
- Start tests
- Stop tests
- Adjust running tests
- Continue paused tests
- Event system
- CLI interface
- Multi-device manager architecture

Planned:

- Status API
- HTTP server
- WebSocket API
- Browser UI
- Data logging
- CSV export
- Test history

## Supported Devices

Currently tested with:

- ZKETECH EBC-A20

Other EBC models may work but have not yet been tested.

## Example CLI

```text
connect 1

start 1 DSC-CC 20000 2500 0

status 1

stop 1

disconnect 1
```

## License

This project is licensed under the MIT License.

## Acknowledgements

This project was inspired by and partially based on the protocol work from
[Kazhuu/ebc-battery-tester](https://github.com/Kazhuu/ebc-battery-tester),
which reverse-engineered and documented the ZKETECH EBC-A20 serial protocol.

The original project is licensed under the MIT License. Relevant protocol and
frame-handling ideas were used as reference while building this Raspberry Pi /
CLI / server-oriented implementation.