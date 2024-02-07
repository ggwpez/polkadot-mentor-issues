pub trait Order {
	fn order(&self) -> u64;
}

impl<T: Order> Order for Option<T> {
	fn order(&self) -> u64 {
		self.as_ref().map_or(9999, |d| d.order())
	}
}

pub trait Colored {
	fn color(&self) -> &str;
}

pub trait Human {
	fn human(&self) -> String;
}

impl<T: Human> Human for Option<T> {
	fn human(&self) -> String {
		self.as_ref().map_or("-".into(), |d| d.human())
	}
}

impl Human for Option<core::time::Duration> {
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
		if self.len() > 60 {
			format!("{}...", &self[..60])
		} else {
			self.clone()
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
