//! [Software Design Document (SDD)](`crate::docs::sdd`) - **How** the project may be implemented.
//!
//! # Table of Contents
//!
//!   [Product](#product)
//!
//!   [Communication](#communication)
//!
//!   [Content](#content)
//!
//! # Product
//!
//! # Communication
//!
//! **trait DataElementIn**
//!
//! *Overview*
//! Item    | Description
//! --------|-----------
//! Purpose | To define an operation which can be added to the database.
//!
//! *
//!     - GetStoredProcedure(): string
//!
//! **trait DataElementOut**
//!
//! Item    | Description
//! --------|-----------
//! Purpose | To define an operation which can be added to the database.
//!
//!
//!     - GetStoredFunction(): string
//!
//! **trait APIElement**
//!
//! Item    | Description
//! --------|-----------
//! Purpose | To define an API for the server and the client.
//!
//!     - GetServerRoute(): Handler
//!     - GetClientRoute(): string
//!
//! **enum RenderResult**
//!
//!     - Default
//!     - Custom(String)
//!
//! # Content
//! We can programmatically add and define types (maybe, or at least define them nicely).
//!  - Server Routes
//!     - Add (POST)
//!     - Remove (DELETE)
//!     - Search (GET) - Different ways
//!     - Update (PUT) - Per editable item
//!  - Client Routes
//!     - Add (POST)
//!     - Remove (DELETE)
//!     - Search (GET) - Different ways
//!     - Update (PUT) - Per editable item
//!  - Stored Functions/Procedures
//!     - Add (INSERT INTO)
//!     - Remove (DELETE FROM)
//!     - Search (SEARCH Function or View)
//!     - Update (UPDATE SET)
//!  - Calls to Such Stored Functions/Procedures
//!     - Add (INSERT INTO)
//!     - Remove (DELETE FROM)
//!     - Search (SEARCH Function or View)
//!     - Update (UPDATE SET)
//!  - Render Content (Client)
//!     - Search (FULL DEFAULT & Other Types per Search)
//!     - Update (Inputs & Client Routes to Such)
//!     - Containers
//!
//!  - All Content defines four things:
//!     - Adds
//!     - Removes
//!     - Searches
//!     - Updatable Items
//!

//!
//! **Content<T>**
//! *Properties*
//!
//!     - Adds: Vec<Add<T>>
//!     - Removes: Vec<Remove<T>>
//!     - Searches: Vec<Search<T>>
//!     - Updates: Vec<Update<T>>
//!     - RequireAuth: bool
//!
//! *Methods*
//!
//!     - DefaultRender(): string
//!
//! **Add<T>**
//!
//!     - GetServerRoute(): string
//!     - GetClientRoute(): string
//!     - GetStoredProcedure(): string
//!     - RequireAuth: bool
//!
//! **Remove<T>**
//!
//!     - GetServerRoute(): string
//!     - GetClientRoute(): string
//!     - GetStoredProcedure(): string
//!     - RequireAuth: bool
//!
//! **Search<T>**
//!
//!     - GetServerRoute(): string
//!     - GetClientRoute(): string
//!     - GetStoredProcedure(): string
//!     - GetRender(): RenderResult
//!     - RequireAuth: bool
//!
//! **Update<T>**
//!
//!     - GetServerRoute(): string
//!     - GetClientRoute(): string
//!     - GetStoredProcedure(): string
//!     - RequireAuth: bool
//!
//! Creation of SQL code & JavaScript?
//!
//! The SDD will go here.

pub mod sdd_doc {}
                                                         