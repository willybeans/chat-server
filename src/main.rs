// procedural macro called tokio::main
// which takes out main function and allows us to run with tokio async boilerplate
use tokio::{
	io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
	net::TcpListener,
};
#[tokio::main]
async fn main() {
	println!("hi!");
	// TcpListener is a return type of impl future, output TcpListener, Error
	// so we need to use .await to get this result out
	// note: unwrap is for basic, and not great error handling :)
	let listener = TcpListener::bind("localhost:8080").await.unwrap();
	loop {
		// accept is a method that accepts a new connection from a TcpListener
		// and yields the connection as well as the address of the connection
		// similar to bind, it returns a future, which outputs a result
		let (mut socket, _addr) = listener.accept().await.unwrap();
		// buffer to put the data into in the form of a stack array
		tokio::spawn(async move {
			//tokio spawn will create a new task
			//moves all of one clients handling into one independent task
			//a lot of languages that have async functions
			//but rust has the concept of an async block
			//so we can wrap an entire function into its own future
			let (reader, mut writer) = socket.split();

			let mut reader = BufReader::new(reader);
			let mut line = String::new();
			loop {
				// loop allows never ending connection
				//socket.read is also async, so it will return result
				// which returns number of bytes that are read from the stream
				let bytes_read = reader.read_line(&mut line).await.unwrap();
				if bytes_read == 0 {
					break;
				}
				//does not write to every socket that is connected
				// instead it writes every single byte in the input buffer out
				// its very common to use write, instead of write all, and naturall
				// intercept calls, and do scheduling
				// but tokio is handling the manual cursor advancing for us
				writer.write_all(line.as_bytes()).await.unwrap();
				line.clear();
			}
		});
	}
}
// rust calls async "future"
// future = a thing that does not have a known value yet, but may in the future
// they seem to be similar to JS promises
