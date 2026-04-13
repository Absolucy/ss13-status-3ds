pub mod colors;
pub mod status;
pub mod topic;

use crate::{
	colors::{BOLD, Color, RESET, ansi, fg},
	status::{ServerStatus, ShuttleMode},
};
use ctru::{
	prelude::*,
	services::gfx::{Flush, Swap},
};
use std::{
	net::Ipv4Addr,
	time::{Duration, Instant},
};

const UPDATE_INTERVAL: Duration = Duration::from_secs(10);
static SERVERS: &[(&str, &str, u16)] = &[
	("Monkestation MRP1", "104.194.9.21", 3121),
	("Monkestation MRP2", "104.194.9.21", 3122),
	("Oculis", "104.194.9.21", 42069),
];

pub fn add_idx(idx: &mut usize, amt: i8) {
	let new_idx = ((*idx as isize) + (amt as isize)).max(0) as usize;
	*idx = if new_idx == 0 {
		SERVERS.len()
	} else if new_idx > SERVERS.len() {
		1
	} else {
		new_idx
	};
}

fn main() {
	let mut apt = Apt::new().unwrap();
	let mut hid = Hid::new().unwrap();
	let gfx = Gfx::new().unwrap();
	let mut top_screen = Console::new(gfx.top_screen.borrow_mut());
	let bottom_screen = Console::new(gfx.bottom_screen.borrow_mut());

	apt.set_app_cpu_time_limit(45)
		.expect("Failed to enable system core");

	top_screen.set_double_buffering(true);
	top_screen.swap_buffers();

	// soc.redirect_to_3dslink(true, true)
	// 	.expect("unable to redirect stdout/err to 3dslink server");

	bottom_screen.select();

	// Owning a living handle to the `Soc` service is required to use network
	// functionalities.
	let _soc = Soc::new().unwrap();

	let mut idx = 1_usize;
	top_screen.flush_buffers();
	top_screen.swap_buffers();

	let mut last_update = Instant::now();
	let mut last_status = fetch_server_status(SERVERS[0], &bottom_screen);
	while apt.main_loop() {
		let mut needs_fetch = false;

		let time_since_fetch = last_update.elapsed();

		hid.scan_input();
		let keys = hid.keys_down();
		if keys.intersects(KeyPad::START | KeyPad::Y) {
			break;
		} else if keys.contains(KeyPad::DPAD_RIGHT) {
			add_idx(&mut idx, 1);
			needs_fetch = true;
		} else if keys.contains(KeyPad::DPAD_LEFT) {
			add_idx(&mut idx, -1);
			needs_fetch = true;
		} else if last_update.elapsed() >= UPDATE_INTERVAL {
			needs_fetch = true;
		}

		if needs_fetch {
			last_status = fetch_server_status(SERVERS[idx - 1], &bottom_screen);
			last_update = Instant::now();
		}

		top_screen.select();
		top_screen.clear();
		if let Some(status) = &last_status {
			display_status(
				SERVERS[idx - 1].0,
				status,
				Duration::from_secs(time_since_fetch.as_secs()),
			);
		}
		top_screen.flush_buffers();
		top_screen.swap_buffers();

		gfx.wait_for_vblank();
	}
}

fn fetch_server_status(
	(_name, ip, port): (&str, &str, u16),
	bottom_screen: &Console,
) -> Option<ServerStatus> {
	let ip: Ipv4Addr = ip.parse().unwrap();
	bottom_screen.select();
	bottom_screen.clear();
	let status = match topic::topic(ip, port, "?status&format=json") {
		Ok(topic) => match serde_json::from_str::<status::ServerStatus>(topic.trim()) {
			Ok(status) => status,
			Err(err) => {
				bottom_screen.select();
				bottom_screen.clear();
				println!("{topic}\n");
				println!(
					"{}{BOLD}decode error:{RESET} {}{err:?}{RESET}",
					fg(Color::Red),
					ansi().fg(Color::White).bg(Color::Red)
				);
				return None;
			}
		},
		Err(err) => {
			bottom_screen.select();
			bottom_screen.clear();
			println!(
				"{}{BOLD}fetch error:{RESET} {}{err:?}{RESET}",
				fg(Color::Red),
				ansi().fg(Color::White).bg(Color::Red)
			);
			return None;
		}
	};
	Some(status)
}

fn display_status(name: &str, status: &status::ServerStatus, time_since_fetch: Duration) {
	println!();
	println!("=== {BOLD}{name}{RESET} ===");
	println!(
		"{BOLD}Players:{RESET}         {}{}{RESET}",
		fg(Color::Green),
		status.players,
	);
	/* println!(
		"{BOLD}Admins:{RESET}          {}{}{RESET}",
		ansi().fg(Color::Red).finish(),
		status.admins,
	); */
	println!("{BOLD}Round ID:{RESET}        {}", status.round_id);
	println!(
		"{BOLD}Game State:{RESET}      {}{:?}{RESET}",
		fg(status.game_state_color()),
		status.game_state
	);
	if status.has_round_started() {
		println!(
			"{BOLD}Round Duration:{RESET}  {}",
			humantime::format_duration(status.round_duration + time_since_fetch)
		);
	}
	println!(
		"{BOLD}Map:{RESET}             {}{}{RESET}",
		fg(Color::Magenta),
		status.map_name,
	);
	println!(
		"{BOLD}Security Level:{RESET}  {}{}{RESET}",
		fg(status.security_color()),
		status.security_level,
	);
	println!(
		"{BOLD}Time Dilation:{RESET}   {}{}%{RESET}",
		fg(status.tidi_color()),
		status.time_dilation.current.round() as u16
	);
	if let Some(shuttle) = &status.shuttle_info {
		println!(
			"{BOLD}Shuttle Status:{RESET}  {}{:?}{RESET}",
			if status.is_shuttle_coming() {
				fg(Color::Magenta)
			} else {
				ansi().fg(Color::Yellow).dim().finish()
			},
			shuttle.shuttle_mode
		);
		if status.is_shuttle_coming() || shuttle.shuttle_mode == ShuttleMode::Recall {
			println!(
				"{BOLD}Shuttle Time:{RESET}    {}{}{RESET}",
				fg(Color::White),
				humantime::format_duration(shuttle.shuttle_timer + time_since_fetch)
			);
		}
		if let Some(reason) = &shuttle.reason {
			println!(
				"{BOLD}Shuttle Reason:{RESET}  {}{:?}{RESET}",
				fg(Color::White),
				reason
			);
		}
	}
}
