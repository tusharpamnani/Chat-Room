// Import necessary modules and traits from Rocket
#[macro_use]
extern crate rocket;
use rocket::{
    tokio::sync::broadcast::{channel, Sender, error::RecvError},
    serde::{Serialize, Deserialize},
    State,
    Shutdown,
    response::stream::{EventStream, Event},
    fs::{relative, FileServer},
};
use rocket::form::Form;
use rocket::tokio::select;

// Define a data structure for a chat message, implementing Serialize, Deserialize, Debug, and FromForm traits
#[derive(Serialize, Deserialize, Debug, Clone, FromForm)]
#[serde(crate = "rocket::serde")]
struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

// Define a Rocket route to handle incoming POST requests for sending messages
#[post("/message", data="<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    // Extract the form data and send it to the message queue
    let _res = queue.send(form.into_inner());
}

// Define a Rocket route to handle incoming GET requests for event streaming
#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    // Subscribe to the message queue to receive new messages
    let mut rx = queue.subscribe();

    // Define an event stream that yields messages to the client
    EventStream! {
        loop {
            // Use the select! macro to handle multiple asynchronous operations
            let msg = select! {
                // Receive a message from the message queue
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break, // Break if the message queue is closed
                    Err(RecvError::Lagged(_)) => continue, // Continue if there is lag in the message queue
                },
                // Check for a shutdown signal
                _ = &mut end => break, // Break if a shutdown signal is received
            };

            // Yield the message as a JSON event to the client
            yield Event::json(&msg);
        }
    }
}

// Define the main Rocket application
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0) // Create a channel for broadcasting messages
        .mount("https://chat-room-alpha-five.vercel.app/", routes![post, events]) // Define routes for handling POST and GET requests
        .mount("/", FileServer::from(relative!("static"))) // Serve static files from the "static" directory
}
