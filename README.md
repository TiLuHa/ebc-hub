# Open Battery Forge

Open-source battery inventory, lifecycle, testing, and device integration platform.

**Open Battery Forge** is a Rust-based application for managing individual battery cells throughout their lifecycle. It combines battery inventory management, intake documentation, test history, measurement storage, and integration with external battery testers in one long-running server application.

The project is designed to run on a Raspberry Pi, server, or desktop machine. Tests and data collection can continue independently of any connected browser or CLI client.

## Project History

Open Battery Forge was originally developed under the name **`ebc-hub`**.

The initial goal of `ebc-hub` was to provide remote control and test management for ZKETECH EBC battery testers. As the project developed, its scope expanded beyond controlling a specific tester.

The application now focuses on the complete lifecycle of batteries and battery cells:

* Registration and inventory
* Delivery and intake measurements
* Test planning and execution
* Measurement and sample storage
* Installation and usage history
* Retesting and condition tracking
* Retirement and long-term traceability

ZKETECH EBC support remains an important part of the project, but it is treated as one device integration rather than the central purpose of the application.

## Goals

* Manage battery types and individual battery cells
* Maintain a complete lifecycle history for every battery
* Record purchasing, delivery, and intake information
* Store test sessions, measurements, and samples
* Run long-lived and autonomous battery tests
* Support multiple battery testers simultaneously
* Provide a device-independent tester interface
* Integrate additional battery testers through drivers
* Provide remote control through HTTP and WebSocket APIs
* Offer a browser-based user interface
* Export test data for analysis and reporting
* Run reliably on small systems such as a Raspberry Pi

## Architecture

Open Battery Forge separates battery management from device-specific communication.

The core application manages domain objects such as:

```text
Battery Type
    ↓
Battery
    ↓
Battery Intake
    ↓
Battery Test
    ↓
Test Session
    ↓
Samples and Results
    ↓
Installation and Lifecycle History
```

Battery testers are connected through device-specific integrations.

The ZKETECH EBC-A20 implementation is currently part of the project. In the future, the protocol and driver code may be moved into a separate Rust crate or repository so that it can also be used independently of Open Battery Forge.

A future driver structure could look like:

```text
Open Battery Forge
├── Core battery management
├── Storage
├── Web interface
├── Tester abstraction
└── Drivers
    ├── ZKETECH EBC-A20
    ├── Additional ZKETECH models
    └── Other battery testers
```

## Current Status

The project is in active development.

### Implemented

* ZKETECH EBC protocol implementation
* EBC-A20 device communication
* Multi-device manager
* Device connect and disconnect
* Start, stop, adjust, and continue commands
* Device event broadcasting
* Interactive CLI
* SQLite database
* SQL migrations
* Type-safe database access using `sqlx`
* Battery type management
* Battery inventory
* Battery intake management
* Initial browser-based battery management interface

### In Progress

* Test management
* Test sessions
* Measurement and sample storage
* Persistent test runner
* HTTP and WebSocket server
* Live device status
* Device-independent tester abstraction
* Separation of EBC-specific driver code from the application core

### Planned

* Complete browser-based user interface
* Live test monitoring
* Test charts and data visualization
* CSV import and export
* Automatic report generation
* REST API
* Authentication and authorization
* Additional battery tester drivers
* Battery installation and retirement tracking
* Docker deployment

## Database

Open Battery Forge uses SQLite for persistent storage.

The database schema is designed to store:

* Battery types
* Individual batteries and cells
* Battery intake information
* Serial numbers and internal identifiers
* Purchasing and delivery information
* Initial voltage and resistance measurements
* Visual inspection results
* Battery tests
* Test sessions
* Measurement samples
* Test results
* Notes and lifecycle information

This allows battery metadata, intake measurements, test data, and lifecycle history to be stored independently of any specific tester.

Database migrations are stored in the `migrations` directory.

Compile-time SQLx query metadata is stored in the `.sqlx` directory and is committed to the repository so that the project can be built without access to a local development database.

Local database files and `.env` files should not be committed.

## Supported Devices

Currently tested with:

* ZKETECH EBC-A20

Support for additional ZKETECH EBC models and other battery testers is planned.

The long-term goal is to expose a common tester interface while keeping protocol-specific implementations in separate drivers.

## Example CLI

```text
battery-type add EVE LF314 LiFePO4 3200 314000

battery add eve-314ah-001 1

battery-intake set eve-314ah-001 3291 180

connect 1

start 1 DSC-CC 20000 2500 0
```

The exact CLI syntax may change while the project is under active development.

## Development

Create a local `.env` file containing the SQLite database URL:

```dotenv
DATABASE_URL=sqlite://open-battery-forge.db
```

Run the migrations:

```bash
cargo sqlx migrate run
```

Update the SQLx query metadata after changing queries or the database schema:

```bash
cargo sqlx prepare
```

Build and test the project:

```bash
cargo check
cargo test
```

Run the application:

```bash
cargo run
```

## Roadmap

* [x] EBC-A20 communication
* [x] Multi-device manager
* [x] SQLite storage
* [x] Battery type management
* [x] Battery inventory
* [x] Battery intake management
* [x] Initial battery management web interface
* [ ] Device-independent tester interface
* [ ] Persistent test runner
* [ ] Test session storage
* [ ] Measurement sample storage
* [ ] HTTP and WebSocket API
* [ ] Live browser monitoring
* [ ] CSV import and export
* [ ] Additional tester drivers
* [ ] Installation and lifecycle tracking
* [ ] Authentication
* [ ] Docker deployment

## Previous Name

This repository was previously named:

```text
ebc-hub
```

The project was renamed to:

```text
open-battery-forge
```

The rename reflects the broader scope of the application. Open Battery Forge is no longer intended to be only an EBC control tool, but a general battery inventory, lifecycle, testing, and device integration platform.

References to `ebc-hub` may still appear in older commits, documentation, configuration examples, or release artifacts.

## License

This project is licensed under the MIT License.

See the `LICENSE` file for details.

## Acknowledgements

The original EBC communication work in this project was inspired by and partially based on the protocol research from:

[Kazhuu/ebc-battery-tester](https://github.com/Kazhuu/ebc-battery-tester)

That project reverse-engineered and documented the serial protocol used by the ZKETECH EBC-A20 battery tester.

Its protocol documentation and implementation served as an important reference while developing the original `ebc-hub` device communication layer. This made it possible to build EBC-A20 support without independently repeating the complete protocol reverse-engineering process.

The original project is licensed under the MIT License. Protocol details and parts of the communication approach were used as a reference in accordance with that license.

The EBC-A20 driver work may later be extracted into a separate Rust crate or repository. The acknowledgement of the original reverse-engineering work will remain part of that driver project as well.
