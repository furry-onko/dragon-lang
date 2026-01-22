#![allow(unused)]
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Link {
	pub root_crate: String,
	pub entrypoint: String,
}

#[derive(Debug, Deserialize)]
pub struct LinkFile {
	pub file: String,
}

#[derive(Debug, Deserialize)]
pub struct Draco {
	pub program: Program,
	pub release: Release,
}

#[derive(Debug, Deserialize)]
pub struct Program {
	pub name: String,
	pub entrypoint: String,
}

#[derive(Debug, Deserialize)]
pub struct Release {
	pub name: String,
	pub entrypoint: String,
	pub version: String
}

#[derive(Debug)]
pub enum Tomls {
	Link,
	LinkFile,
	Draco,
	Program,
	Release,
}