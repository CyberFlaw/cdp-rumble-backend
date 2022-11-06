# Rumble Backend (CDP 2.0 - Task 2)

[This project is still under development. Read me will be modified when deployed 🚀]

This repo consists of the backend code for Rumble Chatroom. This is built for TinkeHub Career Development Program 2.0

The backend is created in Actix Web framework in Rust. I used this specific framework due to the availability of both HTTP and WSS support and its blazing-fast speed, which puts [Node.js and Python Performance](https://www.techempower.com/benchmarks/#section=data-r21) to shame

The backend will be dockerized and will be deployed in a droplet in Digital Ocean or Render according to the time available

**Note:** This project is not complete at the time of submission. The several changes needed are documented inside the codebase itself. An overview will be provided in the below section.

## Overview of Protocols and Technologies used

This server works over MVC architecture. The server is set up in a way it can perform certain actions when an HTTP request is passed to a specific endpoint. Such a program is generally known as REST API. This is used for the majority of the program for workloads like taking requests from web-client and talking with databases etc

HTTP is a Half duplex protocol over TCP, which is a Fully duplex. To enable a chat system, we need consecutive 2-way communication between server and client. So HTTP cant is used because it won't be enough to send messages in real-time. So we upgrade our protocol to WebSockets. WebSockets allow us to do real-time communication which enables our Chat needs. Actix Web provides both protocols to the disposal of the end user and it can be included in a single working application. So HTTP could be used for sending asynchronous requests and WebSockets for real-time communications... At least that was the plan. More on that later

The database used here is MongoDB. It is a non-relational database. It uses documents to store data instead of relations. This was preferred because of its ease of use compared to relational databases and MongoDB also provides an ODM (Object Document Mapper) for popular programming languages as an SDK. SDK for Node.js is superb with excellent documentation and examples and community, but Rust SDK is so much lackluster compared to the Node.js SDK, with poor documentation and very limited community support.

## The Architecture

### Server Code

The server is running an Actix Web app which is built in Rust. The src is divided into 3 parts:

**api** : This directory contains all the routes for the router

**model** : This directory contains all the document models for the MongoDB collections

**repository** : This directory contains all the database operations done on the server side

### Database

Two databases are working in a shared MongoDB Atlas Cluster. The databases are `Users` and `Messages` respectively

![database](/screenshots/database.jpg)

The `Users` database contains 2 collections:

- Registered
- Rooms

`Registered` contains the data of the users authenticated from the front end by Firebase. The schema is shown below

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        // common::serialize_with = "serialize_object_id"
    )]
    pub id: Option<ObjectId>,
    pub name: String,
    pub unqid: u32,     // 6 digit unique number
    pub image: String,
    pub rooms: Vec<ObjectId>,
    pub email: String,
}
```

`Rooms` contain the data of all the rooms present. The schema is shown below

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Rooms {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user1: u32,     // 6 digit unique number
    pub user2: u32,     // 6 digit unique number
    pub name: String,   // "unqid(user1)+unqid(user2)": String
}
```

The name of each room is a string which is the concatenation of 6-digit unique ids of each user

The `Messages` database contains a collection for each room. The collection name is the same as the name of the room

Each collection \<room_name> contains a document that tracks each message in the room. The `Message` schema is shown below

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Messages {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub owner: Option<ObjectId>,    // Object id of the user who send the message
    pub text: String,
}
```

## End Points

- `/` : Health Check

- `/user/register` : To register a user. A JSON should be passed in the body

- `/user/{id}` : To get details of the user

- `/user/all` : To get all the users

- `/join/{user_id}/{friend_id}` : This takes the 6-digit unique ids of 2 users and makes a room and appends it to the room Array in the `Registered` collection. (This will be replaced with a query pattern URL)

- `/room/{room_id}` : Fetched data about a room

- `/echo/{room_id}` : **Not Implemented!** This route handles the chat room logic.

### Side Note about Websocket Route

The route `/echo/{room_id}` only becomes functions only after upgrading the `actix` crate to version `0.13.0` and `actix-web-actors` crate to version `4.1.0` as the sample code I used as reference is build using versions `0.10.0` and `0.3.2` respectively, which requies `actix-web` crate (the crate used for creating the REST API) to be downgraded to version `3.2.0` from version `4.0.0`

Here are a few screenshots of the requests

![health-check](screenshots/api%20health.jpg)

![test-1](screenshots/api%20test.jpg)

![test-2](screenshots/api%20test%202.jpg)

## Problems Faced

A lot of problems have been faced while developing. I'm really new to Rust (1 or 2 weeks of experience), and my concepts are still not solid yet. The fact that I was able to complete this far within a week is a big deal for me. I haven't challenged myself this much in such a long time. In the end, I'm just happy I did this far from level zero within a week

The main reason I chose Rust over Node.js, is the pain of the lack of logging and error handling Node have. Yes, you can configure it with external packages and they are convenient but, it's very easy for me to make bugs in Node.js. Even though I'm familiar with Node.js I'm personally not a big fan of it. Now the actual problems I had when developing this apart from my conceptual and syntactical bottlenecks are

- WebSocket support from Actix Web was something that pulled me into using this framework, but it's the same thing that didn't work for me. Even the sample code in the documentation didn't work for me by using a normal tool like Postman or Hopscotch. I had to use a tool called `wscat` made by the WebSocket team in Node.js for getting a debuggable output.

- MongoDB SDK for Rust is very vaguely documented. The only proper resource I had was a blog, whose methods are outdated in modern async Rust, and a dev conference from 2019. I even had to go into StackOverflow and ask a question to solve a problem, due to the lack of official and unofficial citings of code

## TODO

- Upgrade `actix-web-actors` to version `0.4.0` to enable the WebSocket route. Until then FireBase (Firestore) will be used as the chat system for **Rumble**

- Resolve a few functions more cleanly, instead of workarounds and with full error handling

- Dockerize (if extra time is available)

## How to run the code

- Install Rust and Cargo

- Clone the repo

```bash
git clone https://github.com/CyberFlaw/cdp-rumble-backend.git
```

- Start the application

```bash
cargo run
```

- To start the server in debug mode use, install `cargo watch`

```bash
cargo watch -x 'run app'
```
