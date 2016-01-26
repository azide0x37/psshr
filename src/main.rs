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
//psshr [push | pull | run] <command | filepath | filename> <server_json>

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
	fn new(username: string,
		   password: string,
		   hostip: string,
		   command: string) {
		Server { username: username,
				 password: password,
				 hostip: hostip,
				 command: command }

	}
}

fn connect(command: string, server: Server) {
	// Connect to the local SSH server
	let tcp = TcpStream::connect(server.hostip).unwrap();
	let mut sess = Session::new().unwrap();
	sess.handshake(&tcp).unwrap();
	sess.userauth_password(server.username, server.password).unwrap();
	let mut channel = sess.channel_session().unwrap();

	//Execute our command, pass result to string
	channel.exec(server.command).unwrap();
	let mut s = String::new();
	channel.read_to_string(&mut s).unwrap();
	println!("{}", s);
	println!("{}", channel.exit_status().unwrap());
}

fn populate(command: string, path: string) /*-> Vec<Server>*/ {
	//Initialize Vector of Structs
	let mut vec_Servers = Vec::new();
	
	//open <path.json> file with server information
	let mut file = match File::open(&path) {
		// The `description` method of `io::Error` returns a string that
		// describes the error
		Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
		Ok(file) => file,
	};

    // Read the file contents into ... whatever Rust uses for JSON?
    //let mut s = String::new();

	//parse JSON into list of Server objects
	/*
	for item in len(objects_in_json) {
			vec_Servers.push(convert_json_to_struct(item));
	}*/

	//return Vec of server objects
}

fn parallel(servers: Vec<Server>){
	let mut threads = vec![];

	//spawn thread for each server
	for server in servers {
		threads.push(thread::spawn(connect(server)))
	}
}

fn main(){
	//Just A Server for testing
	let host = Server::new("root",
						   "password",
						   "172.18.119.89",
						   "ls -latr");

	//initialize location of server_list document
	let path = Path::new(filename);

	parallel(populate(command))
}

