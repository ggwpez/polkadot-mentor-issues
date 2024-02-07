use core::time::Duration;
use octocrab::models::IssueId;
use serde::{Deserialize, Serialize};
use std::{
	collections::BTreeMap,
	io::{Read, Write},
	time::{Instant, SystemTime, UNIX_EPOCH},
};
use crate::types::*;

const CACHE_PATH: &str = "data.json";

/// A single github issue.
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
	pub fn title(&self) -> String {
		self.0.title.clone()
	}

	pub fn url(&self) -> String {
		self.0.html_url.as_str().to_string()
	}

	pub fn creator(&self) -> String {
		self.0.user.login.clone()
	}

	pub fn difficulty(&self) -> Option<Difficulty> {
		self.0.labels.iter().find_map(|label| {
			label.name.as_str().try_into().ok()
		})
	}

	pub fn typ(&self) -> Option<IssueType> {
		self.0.labels.iter().find_map(|label| {
			label.name.as_str().try_into().ok()
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
}

/// Collection of github issues.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct Issues {
	pub issues: BTreeMap<IssueId, Issue>,
	/// When this was last fetched.
	pub last_updated: Option<u64>,
}

impl Issues {
	fn now() -> Self {
		Self { last_updated: Some(now_s()), ..Default::default() }
	}

	pub fn since_last_update(&self) -> Option<Duration> {
		Some(Duration::from_secs(now_s() - self.last_updated?))
	}

	pub async fn load() -> Result<Self> {
		let path = std::path::Path::new(CACHE_PATH);
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

	/// Try to load `Self` from a cache filoe.
	fn try_from_cache() -> Result<Self> {
		let mut file = std::fs::File::open(CACHE_PATH)?;
		let mut data = Vec::new();
		file.read_to_end(&mut data)?;
		serde_json::from_slice(&data).map_err(Into::into)
	}

	pub async fn fetch() -> Result<Self> {
		let mut s = Self::now();
		log::info!("Fetching data from remote...");

		s.fetch_issues().await?;

		// Store in a file for faster restarts if it crashed
		let mut file = std::fs::File::create(CACHE_PATH)?;
		let data = serde_json::to_vec(&s)?;
		file.write_all(&data)?;
		log::info!("Data written to {CACHE_PATH}");

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
			.per_page(100)
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
