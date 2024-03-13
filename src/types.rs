// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use crate::traits::*;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// How difficult an issue is estimated to be.
pub enum Difficulty {
	/// Good First Issue
	GoodFirstIssue,
	Easy,
	Medium,
	Difficult,
	Involved,
}

impl TryFrom<&str> for Difficulty {
	type Error = Box<dyn std::error::Error>;

	fn try_from(label: &str) -> Result<Self> {
		match label {
			"C2-good-first-issue" => Ok(Self::GoodFirstIssue),
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
			Self::GoodFirstIssue => "Good First Issue",
			Self::Easy => "Trivial",
			Self::Medium => "Easy",
			Self::Difficult => "Difficult",
			Self::Involved => "Hard",
		}
		.to_string()
	}
}

impl Order for Difficulty {
	fn order(&self) -> u64 {
		match self {
			Self::GoodFirstIssue => 0,
			Self::Easy => 1,
			Self::Medium => 2,
			Self::Difficult => 3,
			Self::Involved => 4,
		}
	}
}

impl Colored for Difficulty {
	fn color(&self) -> &str {
		match self {
			Self::GoodFirstIssue => "green",
			Self::Easy => "green",
			Self::Medium => "orange",
			Self::Difficult => "yellow",
			Self::Involved => "red",
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

impl TryFrom<&str> for IssueType {
	type Error = Box<dyn std::error::Error>;

	fn try_from(label: &str) -> Result<Self> {
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
