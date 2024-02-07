// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

use actix_web::HttpResponse;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "issues.stpl")]
pub struct Issues<'a> {
	pub issues: &'a crate::github::Issues,
}

impl<'a> Issues<'a> {
	pub fn from_issues(issues: &'a crate::github::Issues) -> Self {
		Self { issues }
	}
}

pub(crate) fn http_200<T>(msg: T) -> HttpResponse
where
	String: std::convert::From<T>,
{
	let msg: String = msg.into();
	HttpResponse::Ok().content_type("text/html; charset=utf-8").body(msg)
}
