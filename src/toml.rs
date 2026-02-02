#![allow(unused)]
use serde::Deserialize;
use crate::file;
use crate::visual;
use std::process;

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

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Tomls {
	Link(Link),
	LinkFile(LinkFile),
	Draco(Draco),
}

pub fn parse_toml(path: &str) -> Option<Tomls> {
	// Check if the file is a Draco.toml and not a .ddl link file
	if file::extract_file(path) == Some("draco.toml") && file::location_exists(path) {
		let draco_toml_content: &str = &file::read_file_string(path);
		if let Ok(draco_toml) = toml::from_str::<Draco>(draco_toml_content) {
			return Some(Tomls::Draco(draco_toml));
		}
		else {
			visual::error("The syntax of draco.toml file is incorrect.");
			process::exit(1);
		}
	}
	
	// Check for .ddl files
	if file::extract_file_extension(path) == Some(".ddl") {
		let link_content: &str = &file::read_file_string(path);
		if let Ok(tomls) = toml::from_str::<Tomls>(link_content) {
			Some(tomls)
		}
		else {
			visual::error("The syntax of .ddl link file is incorrect.");
			process::exit(1);
		}
	}

	else {
		visual::error("Invalid file");
		process::exit(1);
	}


}