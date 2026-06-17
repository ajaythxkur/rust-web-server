# Rust Web Server

An example REST API built with Rust, Actix Web, and MongoDB.

This project demonstrates how to:

* Build REST APIs using Actix Web
* Connect to MongoDB from Rust
* Perform CRUD operations with a NoSQL database
* Structure a Rust backend application using services and models
* Serialize and deserialize JSON using Serde

## Features

* Create owners
* Create dogs associated with owners
* Create bookings
* Cancel bookings
* Retrieve all bookings
* Store data in MongoDB

## Environment Variables

Create a `.env` file:

```env
MONGODB_URI=mongodb+srv://<username>:<password>@cluster.mongodb.net/
```

## Run the Application

```bash
cargo run
```

The server starts on:

```text
http://localhost:8585
```
