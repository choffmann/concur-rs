# Concur Rust

concur-rs is a rust version of the [go version](https://github.com/akatranlp/concur) of the npm package concurrently. It allows you to run multiple commands concurrently. It also adds the functionality to run commands before and after the main commands, for infra startup and cleanup.

Secondly it adds the functionality for healthchecks, which are always at the bottom of all logs to ensure that the infra and app is up and running.

## Status
- [x] run commands concurrently
- [ ] add support config file
- [ ] add logger with color support 
- [ ] add healthchecks
- [ ] support run commands before and after
