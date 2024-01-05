# Rocket Chat Room Application

## Overview

This is a simple chat room application built with the [Rocket](https://rocket.rs/) web framework in Rust. The application allows users to post messages to chat rooms and view real-time updates using server-sent events (SSE).

## Features

- **Chat Messages:** Users can post messages to specific chat rooms.
- **Real-Time Updates:** Chat messages are broadcasted to clients in real-time using SSE.
- **Simple UI:** The application includes a minimalistic user interface for posting and viewing messages.

## Prerequisites

Before running the application, make sure you have the following installed:

- [Rust](https://www.rust-lang.org/): The programming language used to build the application.

## Getting Started

1. Clone the repository:

    ```bash
    git clone https://github.com/tusharpamnani/Chat-Room.git
    cd Chat-Room
    ```

2. Build and run the application:

    ```bash
    cargo run
    ```

3. Open your web browser and navigate to [http://localhost:8000](http://localhost:8000) to access the chat room.

## Usage

- **Posting Messages:** Enter your username, select a chat room, and type your message. Click the "Send" button to post your message.

- **Real-Time Updates:** Open multiple browser tabs to the same chat room to see real-time updates as messages are posted.

## Project Structure

- **src/main.rs:** Main entry point for the Rocket application. This also defines the routes, handlers, and state.

- **static/:** Directory containing static files for the frontend.
  - **index.html:** HTML file for the main page.
  - **reset.css:** CSS file for resetting default styles.
  - **style.css:** CSS file for styling the chat room.
  - **script.js:** JavaScript file for client-side functionality.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve the application.

## License

This project is licensed under the [MIT License](LICENSE).
