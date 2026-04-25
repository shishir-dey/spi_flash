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

### Public API

| Function | Description |
|----------|-------------|
| `new` | Creates a new `SpiFlash` instance |
| `manufacturer` | Returns the manufacturer of the device |
| `size` | Returns the size of the device |
| `memory_type` | Returns the memory type of the device |
| `page_count` | Returns the number of pages in the device |
| `sector_count` | Returns the number of sectors in the device |
| `block_count` | Returns the number of blocks in the device |
| `read_page` | Reads a page from the device |
| `read_sector` | Reads a sector from the device |
| `read_block` | Reads a block from the device |
| `write_page` | Writes a page to the device |
| `write_sector` | Writes a sector to the device |
| `write_block` | Writes a block to the device |
| `erase_page` | Erases a page from the device |
| `erase_sector` | Erases a sector from the device |
| `erase_block` | Erases a block from the device |

## License

MIT
