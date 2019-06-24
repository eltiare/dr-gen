use shiplift::{Docker, EventsOptions};
use tokio::prelude::{Future, Stream};

fn main() {
    let docker = Docker::new();
    println!("listening for events");
    let builder = EventsOptions::builder();
    let fut = docker
        .events( &builder.build())
        .filter( |e| e.typ == "container" && (e.action == "kill" || e.action == "stop" || e.action == "start"))
        .for_each( move |e| {
            println!("{:#?}", e);
            docker.containers()
                .get(&e.actor.id)
                .inspect()
                .map(|c| println!("{:#?}", c))
        })
        .map_err(|e| eprintln!("Error: {}", e));
    tokio::run(fut);
}