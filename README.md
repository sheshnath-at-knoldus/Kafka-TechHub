# Running Rust Kafka Services

This guide provides instructions for running the Rust Kafka Consumer Service and the Rust Kafka Producer Service. Before starting these services, you need to set up and run Kafka and Zookeeper servers.

## Setting Up Kafka and Zookeeper

Follow these steps to set up Kafka and Zookeeper servers:

1. **Download Apache Kafka**: Download Apache Kafka from the [official website](https://kafka.apache.org/downloads).

2. **Extract Kafka Archive**: Extract the downloaded Kafka archive to a preferred location on your system.

3. **Start Zookeeper**: Open a terminal and navigate to the Kafka directory. Start Zookeeper using the following command:

    ```bash
    bin/zookeeper-server-start.sh config/zookeeper.properties
    ```

4. **Start Kafka Server**: In a new terminal, navigate to the Kafka directory. Start Kafka server using the following command:

    ```bash
    bin/kafka-server-start.sh config/server.properties
    ```



## Running Rust Kafka Services

Follow these steps to run the Rust Kafka Consumer Service and the Rust Kafka Producer Service:

1. **Clone the Repositories**: Clone the repositories to your local machine:

    ```bash
    https://github.com/sheshnath-at-knoldus/Kafka-TechHub.git
    ```

2. **Navigate to Project Directories**: Move into the project directories:

    ```bash
    cd kafka-consumer-rust
    cd kafka-producer-rust
    ```

3. **Configure Kafka Connection**: Update the `application.config` files in the `src/resource` directories of both projects with the Kafka broker address (`localhost:9092`), topic name (`demo_topic`), and group ID (choose a suitable ID).

4. **Build and Run Consumer Service**:
   - Navigate to the Kafka Consumer Service directory.
   - Build and run the Rust Kafka Consumer Service using Cargo:

     ```bash
     RUST_LOG=info,debug cargo run
     ```

5. **Build and Run Producer Service**:
   - Navigate to the Kafka Producer Service directory.
   - Build and run the Rust Kafka Producer Service using Cargo:

     ```bash
     RUST_LOG=info,debug cargo run
     ```

6. **Verify Message Consumption**: After running both services, verify that messages are being produced by the Kafka Producer Service and consumed by the Kafka Consumer Service.

## Conclusion

Congratulations! You have successfully set up and run the Rust Kafka Consumer Service and Rust Kafka Producer Service. You can now integrate these services into your Rust applications to interact with Apache Kafka.

---

If you encounter any issues or have suggestions for improvement, please refer to the  project repositories and open an issue. Happy coding!

