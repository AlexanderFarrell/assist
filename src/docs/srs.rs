//! [Software Requirement Specification (SRS)](`crate::docs::srs`) - What the project is **required** to do.
//!
//! # Purpose
//! Self-Wiki is built to create a wiki-style format for productivity. It's designed to be enormously freeform, searchable, and powerful as a productivity tool and documentation of ones life.
//!
//! - [ ] The program must be a place to organize ideas and projects. 1
//! - [ ] The program must help me keep track of what I'm doing. 1
//! - [ ] The program must be a place to store inspirational content. 1
//!
//! # Content
//! - [ ] The program must allow content types to be defined. 2
//! - [ ] The program must allow the access to content types to be defined (add, remove, search_methods, update). 2
//! - [ ] The program must be able to link to content types if on. 2
//! - [ ] The server must be able to create API routes to content (with defined access). 2
//! - [ ] The server must be able to link to tables in the database with content. 2
//! - [ ] The client must be able to create API fetches to content (with defined access). 2
//! - [ ] The client must be able to render content for each search method. 2
//! - [ ] The program must be able to add links to other content if appropriate. 2
//!
//! ## Ideas
//! **Overview**
//! - [ ] The user must be able to create an idea. 1
//! - [ ] The user must be able to delete an idea. 1
//! - [ ] The user must be able to search ideas. 1
//! - [ ] The user must be able to add tags to an idea. 1
//! - [ ] The user must be able to have parent/child ideas to an idea, which can be added or removed. 1
//! - [ ] The user must be able to update editable fields of a project. 1
//!
//! ## Projects
//! **Overview**
//! - [ ] The user must be able to create a project.
//! - [ ] The user must be able to delete a project.
//! - [ ] The user must be able to search projects.
//! - [ ] The user must be able to categorize projects.
//! - [ ] The user must be able to add/remove sub-projects from a project.
//! - [ ] The user must be able to get a project by title.
//! - [ ] The user must be able to update editable fields of a project.
//! - [ ] Goals and work must occur in projects.
//! - [ ] Ideas can be linked to projects.
//! - [ ] Projects have versions
//! - [ ] Projects have updates
//! - [ ] Projects have requirements in markdown
//! - [ ] Projects have design in markdown
//!
//! ## Ideas
//! **Overview**
//! - [ ] The user must be able to create a page.
//! - [ ] The user must be able to delete a page.
//! - [ ] The user must be able to search pages.
//! - [ ] The user must be able to quickly get to a page by plugging in its title.
//! - [ ] The user must be able to link to pages.
//! - [ ] The user must be able to add tags to an idea.
//! - [ ] The user must be able to remove tags to an idea.
//! - [ ] The user must be able to have parent/child ideas to an idea, which can be added or removed.
//! - [ ] The user must be able to update editable fields of a project.
//!
//! # Server
//! - [x] The server must statically host an HTML, CSS and JavaScript Page.
//! - [ ] The server must allow for pages to be added (when logged in).
//! - [ ] The server must allow for content to be added (when logged in).
//! - [ ] The server must require user authentication via a blowfish cipher
//! - [ ] The server must connect to a database.
//! - [ ] The server must have routes for all content types
//!
//! # Database
//!

pub mod srs_doc {}
