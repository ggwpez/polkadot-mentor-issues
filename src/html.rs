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

pub(crate) fn http_500(msg: String) -> HttpResponse {
	HttpResponse::InternalServerError()
		.content_type("text/html; charset=utf-8")
		.body(msg)
}

pub(crate) fn http_200<T>(msg: T) -> HttpResponse
where
	String: std::convert::From<T>,
{
	let msg: String = msg.into();
	HttpResponse::Ok().content_type("text/html; charset=utf-8").body(msg)
}
