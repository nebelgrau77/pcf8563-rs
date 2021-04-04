# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

...

## [0.0.4] - 2021-04-04

### Added
- Get and clear Voltage Low detector flag
- Get external clock mode flag status
- Get main clock status (running/stopped)
- Get power-on-reset-override flag status
- `rtc_init` function (clears various control bits, sets timer frequency to power saving mode)
- `disable_all_alarms` wrapped to shut off all alarms at once
- Get timer flag status
- Get timer interrupt status
- Get alarm flag status
- Get alarm interrupt status
- Read the alarm minutes, hours, day and weekday 
- Read the current timer value
- Read century value (0 or 1)
- Set century (0 or 1)
- Get clock output status (enabled/disabled)

### Changed
- Control enum fields changed to On/Off
- `control_clock()` function also uses On/Off instead of Start/Stop
- Century is set to zero when month is set
  

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

## [0.0.1] - 2021-04-02

### Added
- Setting and reading datetime
- Enabling/disabling alarms and alarm interrupt
- Setting alarms
- Enabling/disabling timer and timer interrupt
- Setting timer frequency
- Enabling/disabling clock output
- Setting clock output frequency

[0.0.4]: https://github.com/nebelgrau77/pcf8563-rs/releases/tag/v0.0.4
[0.0.3]: https://github.com/nebelgrau77/pcf8563-rs/releases/tag/v0.0.3
[0.0.2]: https://github.com/nebelgrau77/pcf8563-rs/releases/tag/v0.0.2
[0.0.1]: https://github.com/nebelgrau77/pcf8563-rs/releases/tag/v0.0.1