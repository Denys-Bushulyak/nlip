# NLIP - Natural Language Interaction Protocol

[![Crates.io](https://img.shields.io/crates/v/nlip)](https://crates.io/crates/nlip)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Project Overview

The Natural Language Interaction Protocol (NLIP) is a Rust-based data structure library designed to facilitate communication between AI systems and natural language processing components. It provides a standardized way to represent messages with various formats, making it easier to build and integrate AI agents that can communicate using structured data.

NLIP focuses on creating a protocol for representing natural language interactions in a structured, extensible format that can be used across different AI systems and applications.

## Core Data Structures

### Message

The `Message` struct is the fundamental building block of NLIP communications. It represents a complete message with:

- `Format`: The format type of the message (text, token, structured, binary, location, or generic)
- `Subformat`: A string describing the specific sub-format within the main format
- `Content`: The actual content of the message, represented as a JSON value
- `Submessages`: Optional collection of sub-messages that can be nested within the main message

### SubMessage

The `SubMessage` struct represents a nested or auxiliary message within a main message:

- `Label`: An optional label for identifying the sub-message
- `Format`: The format type of the sub-message
- `Subformat`: A string describing the specific sub-format within the main format
- `Content`: The actual content of the sub-message, represented as a JSON value

### DataMessage

The `DataMessage` wrapper struct provides a convenient way to create messages with a specific message type:

- Contains the message type as a string
- Wraps a `Message` struct

### ControlMessage

The `ControlMessage` wrapper struct is designed for control-related communications:

- Wraps a `Message` struct with additional control-specific formatting
- Always includes `"MessageType":"control"` in its serialized representation

## Format Enumeration

The `Format` enum defines the supported message formats:

- `Text`: Text-based content
- `Token`: Tokenized content (e.g., for tokenization in language models)
- `Structured`: Structured data formats (JSON, XML, etc.)
- `Binary`: Binary content (e.g., images, audio)
- `Location`: Geolocation or spatial data
- `Generic`: Generic format that can represent any content type

## Usage Examples

### Creating a Basic Message

```rust
use nlip::{Format, Message, Value};

let message = Message {
    Format: Format::Text,
    Subformat: "english",
    Content: Value::String("Hello, world!".into()),
    Submessages: None,
};

println!("{}", message);
// Output: {"Format":"text","Subformat":"english","Content":"Hello, world!"}
```

### Creating a Message with Submessages

```rust
use nlip::{Format, Message, SubMessage, Value};

let message = Message {
    Format: Format::Text,
    Subformat: "english",
    Content: Value::String("Main content".into()),
    Submessages: Some(vec![SubMessage {
        Label: Some("context"),
        Format: Format::Structured,
        Subformat: "json",
        Content: Value::String("{\\\"key\\\":\\\"value\\\"}".into()),
    }]),
};

println!("{}", message);
// Output: {"Format":"text","Subformat":"english","Content":"Main content","Submessages":[{"Label":"context","Format":"structured","Subformat":"json","Content":"{\\\"key\\\":\\\"value\\\"}"}]}
```

### Creating Data and Control Messages

```rust
use nlip::{Format, DataMessage, ControlMessage, Value};

// Create a data message
let data_message = DataMessage::new(
    "chat",
    Format::Text,
    "english",
    Value::String("User input".into()),
    None
);

println!("{}", data_message);
// Output: {"MessageType":"chat","Format":"text","Subformat":"english","Content":"User input"}

// Create a control message
let control_message = ControlMessage::new(
    Format::Text,
    "control",
    Value::String("shutdown".into()),
    None
);

println!("{}", control_message);
// Output: {"MessageType":"control","Format":"text","Subformat":"control","Content":"shutdown"}
```

## Features

- **Cross-platform compatibility**: Built with Rust for cross-platform support
- **Extensible design**: Flexible format system that can accommodate new content types
- **Structured data representation**: JSON-like serialization for easy integration with web systems
- **Nested message support**: Submessages allow for hierarchical message structures
- **Type safety**: Strong typing with Rust's type system to prevent runtime errors
- **Memory efficiency**: Uses string references (`&'a str`) to minimize memory overhead
- **Comprehensive testing**: Unit tests covering core functionality

## Building and Testing

### Prerequisites

- Rust 1.70 or later
- Cargo (Rust's package manager)

### Building

```bash
# Clone the repository
git clone https://github.com/Denys-Bushulyak/nlip.git
cd nlip

# Build the project
cargo build
```

### Testing

```bash
# Run tests
cargo test

# Run tests with coverage (if you have llvm-cov installed)
make coverage
```

### Documentation

```bash
# Generate documentation
cargo doc --open
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Contact

For questions or support, please open an issue on the [GitHub repository](https://github.com/Denys-Bushulyak/nlip).