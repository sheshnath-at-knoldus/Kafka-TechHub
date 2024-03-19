# Rust Kafka Producer Service

This repository contains a Kafka producer service implemented in Rust. The service facilitates the integration of Rust applications with Apache Kafka, providing a structured approach for producing messages to Kafka topics.

## Table of Contents

- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Customization](#customization)
- [Dependencies](#dependencies)
- [License](#license)
- [Acknowledgements](#acknowledgements)

## Getting Started

Follow these steps to start using the Rust Kafka Producer Service:



1. **Navigate to the Project Directory**: Move into the project directory:

    ```bash
    cd kafka-producer-rust
    ```

2. **Environment Setup**:
   - Ensure Rust is installed on your system. If not, follow the [official Rust installation guide](https://www.rust-lang.org/tools/install).
   - Configure the `application.config` file in the `src/resource` directory with your Kafka broker address, topic name, and group ID:

     ```plaintext
     kafka_broker=localhost:9092
     topic=demo_topic
     group_id=your_group_id
     ```

3. **Configure Dependencies**:
   - Update the `Cargo.toml` file with the necessary dependencies for your project.

4. **Resource Configuration**:
   - Create a `resource` directory in the project root.

5. **Code Implementation**:
   - Customize the provided template code in `src/main.rs`, `src/config.rs`, `src/model.rs`, and `src/producer.rs` to fit your specific use case.

6. **Build and Run**:
   - Build and run your Rust project using Cargo:

     ```bash
     RUST_LOG=info,debug cargo run
     ```

## Project Structure

- **src/main.rs**: Entry point of the application. Initializes the Kafka producer client and orchestrates message production to the Kafka topic.
- **src/config.rs**: Defines the configuration structure and methods for loading configuration parameters from the `application.config` file.
- **src/model.rs**: Defines the data model used for producing messages to Kafka.
- **src/producer.rs**: Contains the `producer` function responsible for producing messages to the Kafka topic.
- **src/resource/application.config**: Configuration file containing Kafka broker address, topic name, and group ID.

## Customization

- Customize the provided template according to your project requirements.
- Ensure proper error handling and logging are implemented for production use.
- Adapt the code to handle different data types or additional Kafka operations as needed.

## Dependencies

This Kafka producer service relies on the following dependencies:

- `rdkafka`: A Rust wrapper for the librdkafka C library, providing Kafka functionality.
- `hocon`: A library for parsing and working with HOCON (Human-Optimized Config Object Notation) configuration files.
- `lazy_static`: A macro for defining lazy evaluated static variables in Rust.
- `serde`: A framework for serializing and deserializing Rust data structures.

## Acknowledgements

- This template is inspired by the need for integrating Rust applications with Apache Kafka.
- Thanks to the Rust community for providing valuable resources and support.

---

Feel free to modify and enhance this template as required for your projects. If you encounter any issues or have suggestions for improvement, please open an issue on GitHub. Happy coding!

