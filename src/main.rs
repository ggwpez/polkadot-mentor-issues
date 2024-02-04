#![allow(dead_code)] // The caching crate produces "unused" functions…
#![allow(unused_imports)]

use actix_files as fs;
use actix_web::{
	get,
	http::header::{CacheControl, CacheDirective},
	middleware,
	middleware::Logger,
	web,
	web::Data,
	App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use clap::Parser;
use core::time::Duration;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use sailfish::TemplateOnce;
use std::{
	collections::{BTreeMap, BTreeSet},
	path::{Path, PathBuf},
	process::Command,
	sync::RwLock,
};

mod github;
mod html;
use html::*;

#[derive(Debug, Parser, Clone)]
#[clap(author)]
pub(crate) struct MainCmd {
	#[clap(long, short, default_value = "127.0.0.1")]
	pub endpoint: String,

	#[clap(long, short, default_value = "8080")]
	pub port: u16,

	/// PEM format cert.
	#[clap(long, requires("key"))]
	pub cert: Option<String>,

	/// PEM format key.
	#[clap(long, requires("cert"))]
	pub key: Option<String>,
}

#[derive(Default)]
pub struct State {
	issues: github::Issues,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	std::env::set_var("RUST_BACKTRACE", "1");
	env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
	let cmd = MainCmd::parse();

	let endpoint = format!("{}:{}", cmd.endpoint, cmd.port);
	log::info!("Listening to http://{}", endpoint);

	let data = Data::new(RwLock::new(State::default()));
	let d2 = data.clone();

	let server = HttpServer::new(move || {
		App::new()
			.app_data(Data::clone(&data))
			.wrap(middleware::Compress::default())
			.wrap(Logger::new("%a %r %s %b %{Referer}i %Ts"))
			.service(index)
			.service(version)
			.service(twitter)
	})
	.workers(4);

	// Use this single-threaded runtime for spawning since out state is not `Send`.
	actix_web::rt::spawn(async move {
		let mut interval = tokio::time::interval(Duration::from_secs(60 * 60 * 3)); // 6 hrs
																			// first one is free
		interval.tick().await;

		let issues = github::Issues::load().await.unwrap();
		d2.write().unwrap().issues = issues;

		loop {
			interval.tick().await;

			let issues = github::Issues::fetch().await.unwrap();
			d2.write().unwrap().issues = issues;
		}
	});

	let bound_server = if let Some(cert) = cmd.cert {
		let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
		builder
			.set_private_key_file(cmd.key.expect("Checked by clap"), SslFiletype::PEM)
			.unwrap();
		builder.set_certificate_chain_file(cert).unwrap();
		server.bind_openssl(endpoint, builder)
	} else {
		server.bind(endpoint)
	};

	bound_server?.run().await
}

#[get("/")]
async fn index(data: Data<RwLock<State>>) -> impl Responder {
	http_200(html::Issues::from_issues(&data.read().unwrap().issues).render_once().unwrap())
}

#[get("/static/twitter.png")]
async fn twitter() -> impl Responder {
	let img = include_bytes!("../static/twitter.png");

	HttpResponse::Ok().content_type("image/png").body(&img[..])
}

#[get("/version")]
async fn version() -> impl Responder {
	http_200(env!("CARGO_PKG_VERSION"))
}
