# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

...

## [0.0.3] - 2021-04-03

### Added
- Function for setting time only, with the date left unchanged

## [0.0.2] - 2021-04-03

### Added 
- Single functions for enable/disable of various bit flags (alarm components, timer and alarm interrupts, clock output)
- Single function for starting/stopping the clock
- Continuous/pulsating output of the timer interrupt

### Removed
- All the enable_ and disable_ functions, replaced by control_ functions

## [0.0.1] - 2021-04-03

### Added
- Setting and reading datetime
- Enabling/disabling alarms and alarm interrupt
- Setting alarms
- Enabling/disabling timer and timer interrupt
- Setting timer frequency
- Enabling/disabling clock output
- Setting clock output frequency

[0.0.3]: https://github.com/nebelgrau77/pcf8563-rs/releases/tag/v0.0.3
[0.0.2]: https://github.com/nebelgrau77/pcf8563-rs/releases/tag/v0.0.2
[0.0.1]: https://github.com/nebelgrau77/pcf8563-rs/releases/tag/v0.0.1