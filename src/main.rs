use hole_punch_connect::HolePunchConnect;
use std::net::UdpSocket;

fn main() -> Result<(), std::io::Error> {
	let mut buf = String::new();
	std::io::stdin().read_line(&mut buf)?;
	let socket = UdpSocket::new_hole_punched("magnusson.space:13241", buf.trim().as_bytes())?;

	{
		let socket = socket.try_clone()?;
		std::thread::spawn(move || loop {
			buf.clear();
			if let Ok(_len) = std::io::stdin().read_line(&mut buf) {
				socket.send(buf.trim().as_bytes()).unwrap();
			}
		});
	}

	let mut buf = [0u8; 512];
	let _ = std::thread::spawn(move || loop {
		if let Ok(len) = socket.recv(&mut buf) {
			println!("{}", String::from_utf8_lossy(&buf[..len]));
		}
	})
	.join();

	Ok(())
}
