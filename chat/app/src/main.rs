
#![allow(unused_imports)]
use std::error::Error;
use log::error;
use tokio::io;

use iced::{
	Settings,
	Application,
};
pub mod chat;
pub mod app;

use app::{DitherChat, DitherChatSettings};



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	// Init Logger
	//env_logger::Builder::new().filter_level(log::LevelFilter::Warn).init();
	env_logger::init();
	
	//Run GUI
	let settings = DitherChatSettings::create();
	DitherChat::run(settings);
	
	Ok(())
}