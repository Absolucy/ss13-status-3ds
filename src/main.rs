pub mod status;
pub mod topic;

use std::net::Ipv4Addr;

use ctru::prelude::*;

fn main() {
	let mut apt = Apt::new().unwrap();
	let mut hid = Hid::new().unwrap();
	let gfx = Gfx::new().unwrap();
	let top_screen = Console::new(gfx.top_screen.borrow_mut());
	let bottom_screen = Console::new(gfx.bottom_screen.borrow_mut());

	apt.set_app_cpu_time_limit(30)
		.expect("Failed to enable system core");

	// soc.redirect_to_3dslink(true, true)
	// 	.expect("unable to redirect stdout/err to 3dslink server");

	top_screen.select();
	println!("funny topic status doohickey");
	println!("\x1b[29;16HPress Start to exit");

	// Owning a living handle to the `Soc` service is required to use network
	// functionalities.
	let _soc = Soc::new().unwrap();

	let ip: Ipv4Addr = "104.194.9.21".parse().unwrap();
	match topic::topic(ip, 3121, "?status&format=json") {
		Ok(topic) => {
			bottom_screen.select();
			let topic = topic.trim();
			match serde_json::from_str::<status::ServerStatus>(topic) {
				Ok(status) => println!("{status:#?}"),
				Err(err) => {
					println!("{topic}\n");
					println!("decode err!: {err:?}");
				}
			}
		}
		Err(err) => {
			bottom_screen.select();
			println!("err!: {err:?}");
		}
	}

	top_screen.select();

	while apt.main_loop() {
		gfx.wait_for_vblank();

		hid.scan_input();
		if hid.keys_down().contains(KeyPad::START) {
			break;
		}
	}
}
