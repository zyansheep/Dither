
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::error::Error;
use log::error;
use tokio::io;

use cursive::{
	Cursive,
	views::{
		TextView,
		EditView,
		Dialog,
		TextContent,
	},
};
use dither_chat::{Client, Config, DitherChatAction, DitherChatEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	env_logger::Builder::new().filter_level(log::LevelFilter::Info).init(); // Init Logger
	let client = Client::new(Config::development())?;
	//let (tx, rx) = client.connect()?;
	// Run swarm and get join handle + thread channels
	let swarm_handle = client.start();
	
	// Run chat middleware using swarm
	let chat_handle = dither_chat::DitherChat::start(swarm_handle);
	
	/*let yaml = load_yaml!("app.yml");
	let app = App::from_yaml(yaml);
	let matches = app.get_matches();
	match matches.args() {
		//match args here
	}*/

	let mut siv = cursive::default();
	
	siv.add_global_callback('q', |s| {println!("Quitting"); s.quit()});
	
	let content = TextContent::new("");
	let internal_content = content.clone();
	//siv.add_layer(TextView::new("Hello cursive! Press <q> to quit."));
	
	siv.add_layer(
		Dialog::new()
			.title("Enter your username")
			.padding_lrtb(1, 1, 1, 0)
			.content(
				EditView::new()
				.on_submit(move |s, text| {
					
				})
				//.with_name("name")
				.max_content_width(20)
			)
			.button("Ok", |s| {
				let name = s.call_on_name(
					"name",
					|view: &mut EditView| view.get_content(),
				).unwrap();
				show_popup(s, &name);
			}),
	);
	
	siv.run();
	
	/*use io::AsyncBufReadExt;
	let mut stdin = io::BufReader::new(io::stdin()).lines();
	let name = content.get_content().source().to_owned();
	println!("Setting username to: {}", name);
	loop {
		print!("Chat> ");
		if let Some(line) = stdin.next_line().await? {
			tx.send(DitherAction::FloodSub("chat".to_owned(), line)).await?;
		}
	}*/
	Ok(())
}

fn show_popup(s: &mut Cursive, name: &str) {
    if name.is_empty() {
        s.add_layer(Dialog::info("Please enter a name!"));
    } else {
        let content = format!("Hello {}!", name);
        s.pop_layer();
        s.add_layer(Dialog::around(TextView::new(content))
            .button("Quit", |s| s.quit()));
    }
}