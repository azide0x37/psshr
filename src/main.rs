/* psshr :: Parallel SSH 

Alexander Templeton - Thomson Reuters 
					  Elektron Real Time
					  Operations - First Level Support

Arguments:	JSON with server information, 
			command to run, 
			or file to upload,
			or file to download

Operation:	Spawns tasks (threads) for each server to run 
			command on, then collected responses and prints
			them in the order they were requested. 

Complexity:	O(n)
*/

//Stationkeeping imports
use std::error::Error;
use std::env;

//File and IO imports
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
//use std::io::prelude::*;

//SSH imports
extern crate ssh2;
use std::net::{TcpStream};
use ssh2::Session;

//Parallelization imports
//use std::comm::SharedChan;

//Vector imports
use std::vec::Vec;

//JSON processing imports
extern crate serialize;
use serialize::{json};

//command line options
//psshr [push | pull | run] <command | filepath | filename> <server_json_filepath>

//Define our Server structure
//Make it JSON decodable
//#[deriving(Decodable)] //is this needed?
struct Server {
	username: string,
	password: string,
	hostip: string,
	command: string,
}

impl Server {
	fn new(username: &str,
		   password: &str,
		   hostip: &str,
		   command: &str) -> 
		Server {
			username: username.to_string(),
			password: password.to_string(),
			hostip: hostip.to_string(),
			command: command.to_string(),
	}
}

fn connect(server: Server) -> $str {
	// Connect to the local SSH server
	let tcp = TcpStream::connect(server.hostip).unwrap();
	let mut sess = Session::new().unwrap();
	sess.handshake(&tcp).unwrap();
	sess.userauth_password(server.username, server.password).unwrap();
	let mut channel = sess.channel_session().unwrap();

	//Execute our command, pass result to string, return
	channel.exec(server.command).unwrap();
	let mut s = String::new();
	channel.read_to_string(&mut s).unwrap();
	
	//Print for testing
	//println!("{}", s);
	//println!("{}", channel.exit_status().unwrap());
}

fn populate(command: string, path: string) -> Vec<Server> {
	//Initialize Vector of Structs
	let mut vecServers = Vec! [];
	
	//open <path.json> file with server information
	let mut file = match File::open(&path) {
		// The `description` method of `io::Error` returns a string that
		// describes the error
		Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
		Ok(file) => file,
	};
	
	for item in serverList {
		vecServers.Push(Server::new(Json::Deserialize(item)))
	}

    // Read the file contents into ... whatever Rust uses for JSON?
    //let mut s = String::new();

	//parse JSON into list of Server objects
	/*
	for record in json[]{
		vecServers.push(Server::new(json[record][0], json[record][1], json[record][2], json[record][3]);
		)
	}
	*/

	//return Vec of server objects
}

fn parallel(servers: Vec<Server>){
	let mut threads = vec![];

	//spawn thread for each server
	for server in servers {
		threads.push(thread::spawn(move || {
			connect(server); }
			);
		);
	}
}

fn main(){
	//Pull Arguments from stdin
	let argvec = vec![]
	for argument in env::Args()
		argvec.Push(argument)

	//First argument is command 
	if argvec.Pull() == "Run" {

		//second argument is command
		command = argvec.Pull()
		
		//then third argument is filename
		filename = argvec.Pull()
		let path = Path::new(filename);
		//Just an example `Server` for testing
		let host = Server::new("root", "password", "172.18.119.89", "ls -latr");

		parallel(populate(command))
	} else if argvec.Pull() == "Push" {
		//
	} else if argvec.Pull() == "Pull" {
		//pass
	}
}

