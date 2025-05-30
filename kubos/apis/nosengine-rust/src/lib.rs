//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Contributed by: Timothy Scott (tmscott@mix.wvu.edu) and Sebastian Hamel (sebastian.hamel@rockets.utoledo.edu)
//

//! This crate contains wrappers around the NOSEngine API. This crate can only be built in a
//! system that has NOS3 installed.
//!
//! The examples in this crate will not run properly unless the NOSEngine server is running on
//! `tcp://localhost:12001`. This is the way it is configured to run in the NOS3 VM.
//!
//! # Example Usage
//!
//! ### Simple send and receive
//!
//! ```norun
//! # extern crate nosengine_rust;
//! # use nosengine_rust::client;
//! # use nosengine_rust::ffi;
//! let bus = client::Bus::new("testbus", "tcp://localhost:12001").unwrap();
//! let node1 = client::DataNode::new(&bus, "node1").unwrap();
//! let node2 = client::DataNode::new(&bus, "node2").unwrap();
//!
//! node1
//!     .send_message("node2", &[1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10])
//!     .unwrap();
//! let result = node2.receive_message().unwrap();
//! assert_eq!(result.get_contents(), &[1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
//! ```
//!
//! ### Using callbacks
//!
//! ```norun
//! # extern crate nosengine_rust;
//! # use nosengine_rust::client;
//! # use nosengine_rust::ffi;
//! let bus = client::Bus::new("testbus", "tcp://localhost:12001").unwrap();
//! let node3 = client::DataNode::new(&bus, "node3").unwrap();
//! let node4 = client::DataNode::new(&bus, "node4").unwrap();
//!
//! extern "C" fn callback(
//!     _data_node: *mut ffi::DataNodeHandle,
//!     msg_ptr: *mut ffi::MessageHandle,
//! ) {
//!     println!("Received message in callback: {:?}", unsafe {
//!         client::Message::get_contents_from_ptr(msg_ptr)
//!     });
//! }
//!
//! node4.set_message_callback(callback);
//!
//! node3.send_message("node4", &[1u8, 2, 3, 4, 5]).unwrap();
//! ```
//!
//! ### UART
//!
//! ```norun
//! # extern crate nosengine_rust;
//! # use nosengine_rust::client::uart::*;
//! let uart1 = UART::new("uart10", "tcp://localhost:12001", "testuart", 15).unwrap();
//! let mut uart2 = UART::new("uart11", "tcp://localhost:12001", "testuart", 15).unwrap();
//!
//! uart2.set_callback(move |data: &[u8]|{
//!     assert_eq!(data, &[1u8, 2, 3, 4]);
//! });
//!
//! uart1.write(&[1u8, 2, 3, 4]);
//! ```

#![deny(missing_docs)]

extern crate libc;

pub mod client;
#[allow(clippy::all)]
pub mod ffi;
