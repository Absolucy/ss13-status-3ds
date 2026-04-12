use anyhow::{Context, Result, anyhow};
use std::{
	io::{Read, Write},
	net::{IpAddr, TcpStream},
};

pub fn craft_topic(query: &str) -> Vec<u8> {
	const BASE_LENGTH: usize = 2 + std::mem::size_of::<u16>() + 6;
	const MAX_LENGTH: usize = u16::MAX as usize - 6;

	let query_len = query.len();
	assert!(
		query_len <= MAX_LENGTH,
		"query too long ({} bytes, max {})",
		query_len,
		MAX_LENGTH
	);
	let mut bytes = Vec::with_capacity(BASE_LENGTH + query_len);
	bytes.extend_from_slice(&[0x00, 0x83]);
	bytes.extend_from_slice(&(query_len as u16 + 6).to_be_bytes());
	bytes.extend_from_slice(&[0x00; 5]);
	bytes.extend_from_slice(query.as_bytes());
	bytes.push(0x00);
	bytes
}

pub fn topic(ip: impl Into<IpAddr>, port: u16, query: impl AsRef<str>) -> Result<String> {
	let ip = ip.into();
	let crafted_topic = craft_topic(query.as_ref());

	eprintln!("connecting");
	let mut connection = TcpStream::connect((ip, port)).context("failed to connect")?;
	eprintln!("connected");
	connection
		.write_all(&crafted_topic)
		.context("failed to write topic")?;
	eprintln!("wrote topic");

	let mut buf = [0_u8; 4096];
	let bytes_read = connection.read(&mut buf).context("failed to read bytes?")?;
	if bytes_read < 7 {
		return Err(anyhow!(
			"too few bytes read (got {bytes_read}, should be at least 7)"
		));
	}
	eprintln!("read {bytes_read} bytes");
	String::from_utf8(buf[5..bytes_read - 1].to_vec()) // cut off the trailing null byte
		.context("failed to convert response to string")
}
