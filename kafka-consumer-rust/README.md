# Rust Kafka Consumer Service

This repository contains a Kafka consumer service implemented in Rust. The service allows you to consume messages from a Kafka topic using the `rdkafka` library.

## Table of Contents

- [Introduction](#introduction)
- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Customization](#customization)
- [Dependencies](#dependencies)


## Introduction

The Rust Kafka Consumer Service facilitates the integration of Rust applications with Apache Kafka, providing a structured approach for consuming messages from Kafka topic.

## Getting Started

Follow these steps to start using the Rust Kafka Consumer Service:

1. **Navigate to the Project Directory**: Move into the project directory:

    ```bash
    cd kafka-consumer-rust
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

4. **Code Implementation**:
   - Customize the provided template code in `src/main.rs`, `src/config.rs`, `src/models.rs`, and `src/consumer.rs` to fit your specific use case.

5. **Build and Run**:
   - Build and run your Rust project using Cargo:

     ```bash
     RUST_LOG=info,debug cargo run
     ```

## Project Structure

- **src/main.rs**: Entry point of the application. Initializes the Kafka consumer client and orchestrates message consumption from the Kafka topic.
- **src/config.rs**: Defines the configuration structure and methods for loading configuration parameters from the `application.config` file.
- **src/models.rs**: Defines the data model used for consuming messages from Kafka.
- **src/consumer.rs**: Contains the `consumer` function responsible for consuming messages from the Kafka topic.
- **src/resource/application.config**: Configuration file containing Kafka broker address, topic name, and group ID.

## Customization

- Customize the provided template according to your project requirements.
- Ensure proper error handling and logging are implemented for production use.
- Adapt the code to handle different data types or additional Kafka operations as needed.

## Dependencies

This Kafka consumer service relies on the following dependencies:

- `rdkafka`: A Rust wrapper for the librdkafka C library, providing Kafka functionality.
- `hocon`: A library for parsing and working with HOCON (Human-Optimized Config Object Notation) configuration files.
- `lazy_static`: A macro for defining lazy evaluated static variables in Rust.
- `serde`: A framework for serializing and deserializing Rust data structures.
