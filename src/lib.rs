use std::{error, fmt};
use std::fmt::{Display, Formatter};

use http::Uri;
use url::Url;

#[derive(Debug)]
pub enum Error {
	InvalidAuthority,
	InvalidHost,
	Http(http::Error),
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Error::InvalidAuthority => f.write_str("invalid authority"),
			Error::InvalidHost => f.write_str("invalid_host"),
			Error::Http(error) => error.fmt(f),
		}
	}
}

impl error::Error for Error {}

pub fn url_to_uri(url: Url) -> Result<Uri, Error> {
	if !url.has_authority() {
		return Err(Error::InvalidAuthority);
	}
	if !url.has_host() {
		return Err(Error::InvalidHost);
	}

	let scheme = url.scheme();

	let username = url.username();
	let mut has_user_info = !username.is_empty();
	let host = url.host_str().unwrap();

	let mut authority = String::with_capacity(username.len() + host.len());
	authority.push_str(username);
	if let Some(password) = url.password() {
		authority.push(':');
		authority.push_str(password);

		has_user_info = true;
	}
	if has_user_info {
		authority.push('@');
	}
	authority.push_str(host);
	if let Some(port) = url.port() {
		authority.push(':');
		authority.push_str(&port.to_string());
	}

	let mut path_and_query = String::from(url.path());
	if let Some(query) = url.query() {
		path_and_query.push('?');
		path_and_query.push_str(query);
	}
	if let Some(fragment) = url.fragment() {
		path_and_query.push('#');
		path_and_query.push_str(fragment);
	}

	let builder = Uri::builder().scheme(scheme).authority(authority).path_and_query(path_and_query);

	builder.build().map_err(Error::Http)
}
