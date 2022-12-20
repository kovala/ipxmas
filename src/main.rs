use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Mutex;
use std::time::Instant;
use actix_web::{App, HttpRequest, HttpServer, Responder, web};
use itertools::Itertools;
use rayon::prelude::*;
use structopt::StructOpt;
use serde::Deserialize;

const CSV_FILE: &str = ".data/ips.csv";

fn s2ipi(s: &str) -> u32 {
	let (a,b,c,d) = s.split('.').map(|s|s.parse::<u32>().unwrap()).collect_tuple().unwrap();
	(a<<24) | (b<<16) | (c<<8) | d
}

struct IpCountry {
	iphi: u32,
	place: String,
}

fn csv_to_map3(file:&str) -> Result<BTreeMap<u32, IpCountry>, anyhow::Error> {
	let file = File::open(file)?;
	let lines:Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

	let map:BTreeMap<u32, IpCountry> = lines
		.par_iter()
		.map(|s| {
			let v = s.split(',').map(|s| s.replace("\"", "")).collect::<Vec<String>>();
			let (lo,hi) = (v[0].parse::<u32>().unwrap(), v[1].parse::<u32>().unwrap());
			(lo, IpCountry{iphi:hi, place:format!("{}/{}/{}", v[3],v[4],v[5])})
		})
		.collect();

	println!("len {}", map.len());
	Ok(map)
}

fn geocode<'a>(map: &'a BTreeMap<u32, IpCountry>, ip: &str) -> Option<(u32, u32, &'a IpCountry)> {
	let key = s2ipi(ip);
	let p = map.range(..key).next_back();
	match p {
		Some((iplo, cc1)) => if key <= cc1.iphi {
			Some((*iplo, key, cc1))
		} else if let Some(cc2) = map.get(&(cc1.iphi + 1)) {
			let knext = cc1.iphi + 1;
			Some((knext, key, cc2))
		} else {
			None
		},
		None => None,
	}
}
fn geocode_fmt(map: &BTreeMap<u32, IpCountry>, ip: &str) -> String {
	match geocode(map, ip) {
		Some((_iplo, _key, cc)) => format!("{}", cc.place),
		None => format!("{}", ip)
	}
}
fn geocode_ips(map: &BTreeMap<u32, IpCountry>, ips: Vec<&str>) -> String {
	ips.iter().map(|ip| geocode_fmt(&map, ip)).join(",")
}

struct AppState {
	map: Box<BTreeMap<u32, IpCountry>>,
}
async fn refresh(data:web::Data<Mutex<AppState>>, _req:HttpRequest) -> impl Responder {
	let start = Instant::now();

	let mut d = data.lock().unwrap();
	d.map.clear();
	*d.map = csv_to_map3(CSV_FILE).expect("csv file");

	let duration = start.elapsed();

	format!("loaded ip-map size:{}! in {:?}", d.map.len(), duration)
}

#[derive(Deserialize)]
struct Ips {
	value: String,
}
async fn ips(data:web::Data<Mutex<AppState>>, q:web::Query<Ips>) -> impl Responder {
	let ips = q.value.split(',').collect::<Vec<&str>>();
	let d = data.lock().unwrap();
	geocode_ips(&d.map, ips)
}

pub fn run_cli() {
	let ip = s2ipi("192.168.1.1");
	println!("ip:{}", ip);

	let m = csv_to_map3(CSV_FILE).expect("csv file");
	let places = geocode_ips(&m, vec!["141.101.90.17","197.234.221.0","67.185.139.0"]);
	println!("{}", places)
}

pub async fn run_web() -> std::io::Result<()> {
	let data = web::Data::new(Mutex::new(AppState{map:Box::new(BTreeMap::new())}));
	let bind = ("127.0.0.1", 8668);
	println!("listening on {:?}", bind);

	HttpServer::new(move|| {
		App::new().app_data(data.clone())
			.route("/refresh", web::get().to(refresh))
			.route("/ips", web::get().to(ips))
	})
	.bind(bind)?
	.run()
	.await
}

#[derive(StructOpt)]
enum Cmd {
	IpWeb,Ip
}
#[derive(StructOpt)]
struct Args {
	#[structopt(subcommand)]
	cmd: Option<Cmd>,
}
#[actix_web::main]
#[paw::main]
async fn main(a: Args) -> anyhow::Result<()> {
	match a.cmd {
		Some(Cmd::Ip) => run_cli(),
		Some(Cmd::IpWeb) => run_web().await?,
		None => {}
	}
	Ok(())
}