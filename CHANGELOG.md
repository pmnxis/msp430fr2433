# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [v0.2.0]
- Regenerated with svd2rust 0.37.1: 
  - (Breaking) Peripheral names have changed from `SCREAMING_SNAKE` to `snake_case`, and type names have changed to `PascalCase`, dropping any underscores.
  - (Breaking) All registers are now accessed through methods. Previously registers were sometimes fields and sometimes methods (based on whether the hardware address was aliased by multiple registers).
- (Breaking) Rename some registers to match user guide. Ex: GPIO registers: `PORT_1_2` --> `PA`. `PORT_3` --> `P3`.
- (Breaking) Replace some generic field enums with user-friendly versions (e.g. `Flld` variants names are now the clock division ratio instead of just `Flld0`, `Flld1`, etc.).
- Add several missing registers
- Added individual `P1` and `P2` GPIO peripherals. Previously they could only be accessed all together via `PA`.

## [v0.1.0] - 2025-10-30
- Initial release
