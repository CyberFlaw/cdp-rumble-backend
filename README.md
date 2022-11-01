# Rumble Backend (CDP 2.0)

[This project is still under development. Read me will be modified when deployed 🚀]

This repo consists of the backend code for Rumble Chatroom. This is built for TinkeHub Career Development Program 2.0

The backend is created in Actix Web framework in Rust. I used this specific framework due to the availability of both HTTP and WSS support and its blazing-fast speed, which puts Node.js and Python to shame. This can also be equipped with an SSL certificate out of the box so that I don't have to configure it in DevOps while deploying or dockerization

The backend will be dockerized and will be deployed in a droplet in Digital Ocean

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
