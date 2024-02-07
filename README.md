# Rust Channels

This is a small Rust program that demonstrates the usage of channels for inter-thread communication.

## Description

The program creates two classes of threads, some server threads that accept transfer commands for a ficitve bank anda given number of client threads sending transfer jobs to the server. The communication is established by a Rust channel. The server threads lock the complete vector of accounts to guarantee consistency.

## Usage

To run the program, make sure you have Rust installed on your system. Then, navigate to the project directory and execute the following command:

`cargo run <number of accounts> <number of server threads> <number of client threads> <number of transactions>`