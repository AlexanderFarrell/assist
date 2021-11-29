//! # Introduction
//! Self-Wiki is built to create a wiki-style format for productivity. It's designed to be enormously freeform, searchable, and powerful as a productivity tool and documentation of ones life.
//! \
//! \
//! **Documents**
//! - [Software Requirement Specification (SRS)](`docs::srs`)
//! - [Software Design Document (SDD)](`docs::sdd`)

pub mod docs;
pub mod auto;
pub mod content;

#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::{Build, Rocket};

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount(
        "/",
        FileServer::from("public"))
}