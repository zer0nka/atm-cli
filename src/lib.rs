// lib.rs
//
// Copyright (c) 2019 All The Music, LLC
//
// This work is licensed under the Creative Commons Attribution 4.0 International License.
// To view a copy of this license, visit http://creativecommons.org/licenses/by/4.0/ or send
// a letter to Creative Commons, PO Box 1866, Mountain View, CA 94042, USA.

//! `atm-cli` is a command line tool for generating and working with MIDI files. It was purpose-built for
//! All the Music, LLC to assist in its mission to enable musicians to make all of their music
//! without the fear of frivolous copyright lawsuits. All code is released into the public domain
//! via the [Creative Commons Attribute 4.0 International License](http://creativecommons.org/licenses/by/4.0/).
//! If you're looking for a Rust library to generate and work with MIDI files, check out
//! [the `libatm` project](https://github.com/allthemusicllc/libatm), on which this tool relies. For
//! more information on All the Music, check out [allthemusic.info](http://allthemusic.info).

extern crate clap;
extern crate flate2;
extern crate itertools;
extern crate libatm;
extern crate pbr;
extern crate tar;

#[doc(hidden)]
pub mod cli;
#[doc(hidden)]
pub mod directives;
pub mod utils;
