# spi_flash

A no_std spi flash driver for Winbond W25Q and similar devices

## Project Structure:

| Module | Description |
|--------|-------------|
| device | Device-specific logic (manufacturer detection, status registers, etc.) |
| error | Error types for the library |
| interface | Trait definitions for SPI interface, CS pin, and delay |
| misc | Miscellaneous utility functions (address conversions, etc.) |
| mock | Mock implementations for testing (only available when "std" feature is enabled) |
| types | Type definitions for commands, status enums, and sizes |

## Usage

Add this as a dependency in your `Cargo.toml`:

```toml
[dependencies]
spi_flash = { path = "path/to/spi_flash" }
```

## License

MIT
