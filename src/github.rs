use core::time::Duration;
use octocrab::models::IssueId;
use serde::{Deserialize, Serialize};
use std::{
	collections::BTreeMap,
	io::{Read, Write},
	time::{Instant, SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Issue(octocrab::models::issues::Issue);

impl PartialEq for Issue {
	fn eq(&self, other: &Self) -> bool {
		self.0.id == other.0.id
	}
}
impl Eq for Issue {}

impl Ord for Issue {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.0.id.cmp(&other.0.id)
	}
}

impl PartialOrd for Issue {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Issue {
	pub fn name(&self) -> Option<String> {
		Some(self.0.title.clone())
	}

	pub fn title(&self) -> String {
		self.0.title.clone()
	}

	pub fn link(&self) -> String {
		self.0.html_url.as_str().to_string()
	}

	pub fn creator(&self) -> String {
		self.0.user.login.clone()
	}

	pub fn typ(&self) -> Option<IssueType> {
		self.0.labels.iter().find_map(|label| {
			let label = label.name.as_str();
			IssueType::try_from_label(label).ok()
		})
	}

	pub fn status(&self) -> Option<Status> {
		let assigned = self.0.assignee.is_some() || !self.0.assignees.is_empty();
		let wip = self.0.pull_request.is_some();

		if wip {
			Some(Status::Wip)
		} else if assigned {
			Some(Status::Taken)
		} else {
			Some(Status::Free)
		}
	}

	pub fn difficulty(&self) -> Option<Difficulty> {
		self.0.labels.iter().find_map(|label| {
			let label = label.name.as_str();
			Difficulty::try_from_label(label).ok()
		})
	}
}

pub trait Order {
	fn order(&self) -> u64;
}

impl<T: Order> Order for Option<T> {
	fn order(&self) -> u64 {
		self.as_ref().map_or(9999, |d| d.order())
	}
}

pub trait Human {
	fn human(&self) -> String;
}

pub trait Colored {
	fn color(&self) -> &str;
}

pub enum Difficulty {
	Easy,
	Medium,
	Difficult,
	Involved,
}

impl Order for Difficulty {
	fn order(&self) -> u64 {
		match self {
			Self::Easy => 0,
			Self::Medium => 1,
			Self::Difficult => 2,
			Self::Involved => 3,
		}
	}
}

impl Colored for Difficulty {
	fn color(&self) -> &str {
		match self {
			Self::Easy => "green",
			Self::Medium => "orange",
			Self::Difficult => "yellow",
			Self::Involved => "red",
		}
	}
}

pub trait RemoveNonAlphaNum {
	fn remove_non_alphanum(&self) -> String;
}

impl RemoveNonAlphaNum for String {
	fn remove_non_alphanum(&self) -> String {
		self.chars().filter(|c| c.is_alphanumeric()).collect()
	}
}

pub trait Sanitize {
	fn sanitize(&self) -> String;
}

impl Sanitize for String {
	fn sanitize(&self) -> String {
		self.replace(['\'', '`', '\"'], "")
	}
}

pub trait Shortened {
	fn shortened(&self) -> String;
}

impl Shortened for String {
	fn shortened(&self) -> String {
		if self.len() > 50 {
			format!("{}...", &self[..50])
		} else {
			self.clone()
		}
	}
}

pub enum IssueType {
	Bug,
	Tests,
	Cleanup,
	Refactor,
	Feature,
	Docs,
	Benchmarking,
}

impl IssueType {
	pub fn try_from_label(label: &str) -> Result<Self> {
		match label {
			"I0-panic" | "I1-security" | "I2-bug" | "I3-annoyance" => Ok(Self::Bug),
			"I4-refactor" | "I9-optimisation" => Ok(Self::Refactor),
			"I5-enhancement" => Ok(Self::Feature),
			// best effort since we have no clue what it is:
			"T10-tests" => Ok(Self::Tests),
			"T13-deprecation" | "T14-cleanup" => Ok(Self::Cleanup),
			"T11-documentation" => Ok(Self::Docs),
			"T12-benchmarks" => Ok(Self::Benchmarking),
			_ => Err(format!("Unknown issue type label: {}", label).into()),
		}
	}
}

impl Order for IssueType {
	fn order(&self) -> u64 {
		match self {
			Self::Bug => 0,
			Self::Tests => 1,
			Self::Cleanup => 2,
			Self::Refactor => 3,
			Self::Feature => 4,
			Self::Docs => 5,
			Self::Benchmarking => 6,
		}
	}
}

impl Human for IssueType {
	fn human(&self) -> String {
		match self {
			Self::Bug => "Fix",
			Self::Tests => "Testing",
			Self::Cleanup => "Cleanup",
			Self::Refactor => "Refactor",
			Self::Feature => "Feature",
			Self::Docs => "Docs",
			Self::Benchmarking => "Benchmarking",
		}
		.to_string()
	}
}

impl Colored for IssueType {
	fn color(&self) -> &str {
		""
	}
}

pub enum Status {
	Free,
	Taken,
	Wip,
}

impl Order for Status {
	fn order(&self) -> u64 {
		match self {
			Self::Free => 0,
			Self::Taken => 1,
			Self::Wip => 2,
		}
	}
}

impl Human for Status {
	fn human(&self) -> String {
		match self {
			Self::Free => "Free",
			Self::Taken => "Taken",
			Self::Wip => "WIP",
		}
		.to_string()
	}
}

impl Colored for Status {
	fn color(&self) -> &str {
		match self {
			Self::Free => "green",
			Self::Taken | Self::Wip => "orange",
		}
	}
}

impl<T: Human> Human for Option<T> {
	fn human(&self) -> String {
		self.as_ref().map_or("-".into(), |d| d.human())
	}
}

pub trait ColoredHuman {
	fn colored_human(&self) -> String;
}

impl<T: Colored + Human> ColoredHuman for T {
	fn colored_human(&self) -> String {
		format!("<span style=\"color:{}\">{}</span>", self.color(), self.human())
	}
}

impl<T: Colored + Human> ColoredHuman for Option<T> {
	fn colored_human(&self) -> String {
		self.as_ref().map_or("-".to_string(), |d| d.colored_human())
	}
}

impl Difficulty {
	pub fn try_from_label(label: &str) -> Result<Self> {
		match label {
			"D0-easy" => Ok(Self::Easy),
			"D1-medium" => Ok(Self::Medium),
			"D2-substantial" => Ok(Self::Difficult),
			"D3-involved" => Ok(Self::Involved),
			_ => Err(format!("Unknown difficulty label: {}", label).into()),
		}
	}
}

impl Human for Difficulty {
	fn human(&self) -> String {
		match self {
			Self::Easy => "Trivial",
			Self::Medium => "Easy",
			Self::Difficult => "Difficult",
			Self::Involved => "Hard",
		}
		.to_string()
	}
}

impl Human for Option<Duration> {
	fn human(&self) -> String {
		self.as_ref().map_or("?".into(), |d| {
			let s = d.as_secs();

			if s < 60 {
				format!("{}s", s % 60)
			} else if s < 3600 {
				format!("{}m", s / 60)
			} else if s < 86400 {
				format!("{}h", s / 3600)
			} else {
				format!("{}d", s / 86400)
			}
		})
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct Issues {
	pub issues: BTreeMap<IssueId, Issue>,
	pub last_updated: Option<u64>,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Issues {
	fn now() -> Self {
		Self { last_updated: Some(now_s()), ..Default::default() }
	}

	pub fn since_last_update(&self) -> Option<Duration> {
		Some(Duration::from_secs(now_s() - self.last_updated?))
	}

	pub async fn load() -> Result<Self> {
		let path = std::path::Path::new("data.json");
		if path.exists() {
			log::info!("Loading from cache...");

			match Self::try_from_cache() {
				Ok(s) => {
					log::info!("Loaded from cache");
					return Ok(s)
				},
				Err(e) => log::warn!("Failed to load from cache. Falling back to fetch: {}", e),
			}
		} else {
			log::info!("Path {} does not exist. Falling back to fetch", path.display());
		}

		Self::fetch().await
	}

	fn try_from_cache() -> Result<Self> {
		let mut file = std::fs::File::open("data.json")?;
		let mut data = Vec::new();
		file.read_to_end(&mut data)?;
		serde_json::from_slice(&data).map_err(Into::into)
	}

	pub async fn fetch() -> Result<Self> {
		let mut s = Self::now();
		log::info!("Fetching data from remote...");

		s.fetch_issues().await?;

		// Store in a file for faster restarts if it crashed
		let mut file = std::fs::File::create("data.json")?;
		let data = serde_json::to_vec(&s)?;
		file.write_all(&data)?;
		log::info!("Data written to data.json");

		Ok(s)
	}

	/// Query each profile description and check if the address is mentioned.
	async fn fetch_issues(&mut self) -> Result<()> {
		log::info!("Fetching github issues...");
		let mut interval = tokio::time::interval(Duration::from_millis(2000));

		interval.tick().await;
		let octocrab = octocrab::instance();

		let mut page = octocrab
			.issues("paritytech", "polkadot-sdk")
			.list()
			.labels(&["C1-mentor".into()])
			.state(octocrab::params::State::Open)
			.per_page(50)
			.send()
			.await?;

		loop {
			for issue in &page {
				log::info!("Fetched issue {}", issue.id.0);
				self.issues.insert(issue.id, Issue(issue.clone()));
			}
			interval.tick().await;

			page = match octocrab.get_page::<octocrab::models::issues::Issue>(&page.next).await? {
				Some(next_page) => next_page,
				None => break,
			}
		}

		Ok(())
	}
}

fn now_s() -> u64 {
	let start = SystemTime::now();
	let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");

	since_the_epoch.as_secs()
}
