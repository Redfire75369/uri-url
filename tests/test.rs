use std::error;

use url::Url;

use uri_url::{Error, url_to_uri};

macro_rules! test_url_to_uri {
	($test_name:ident, $str:expr, $authority:expr, $path_query:expr $(, $final_str:expr)? $(,)?) => (
		#[test]
		fn $test_name() -> Result<(), Box<dyn error::Error>>{
			let url = Url::parse($str)?;
			let uri = url_to_uri(url.clone())?;

			assert_eq!(uri.scheme().unwrap(), url.scheme(), "Different Schemes");
			assert_eq!(uri.authority().unwrap().as_str(), $authority, "Different Authorities");
			assert_eq!(uri.path(), url.path(), "Different Paths");
			assert_eq!(uri.query(), url.query(), "Different Queries");
			assert_eq!(uri.path_and_query().unwrap().as_str(), $path_query, "Different Path and Query");

			$(
				assert_eq!(uri, $final_str, "Different String");
				return Ok(());
			)?

			#[allow(unreachable_code)]
			{
				assert_eq!(uri, $str, "Different String");

				Ok(())
			}
		}
	);
}

macro_rules! test_url_to_uri_fail {
	($test_name:ident, $str:expr $(, $error:expr)? $(,)?) => (
		#[test]
		fn $test_name() -> Result<(), Box<dyn error::Error>>{
			let url = Url::parse($str)?;
			let error = url_to_uri(url.clone()).unwrap_err();

			$(
				assert_eq!(error.to_string(), $error.to_string(), "Different Error");
			)?
			Ok(())
		}
	);
}

test_url_to_uri!(test_url_to_uri, "http://127.0.0.1", "127.0.0.1", "/",);

test_url_to_uri!(test_url_to_uri_path, "http://127.0.0.1/chunks", "127.0.0.1", "/chunks",);

test_url_to_uri!(test_url_to_uri_port, "http://127.0.0.1:61761", "127.0.0.1:61761", "/",);

test_url_to_uri!(test_url_to_uri_https, "https://127.0.0.1", "127.0.0.1", "/",);

test_url_to_uri!(test_url_to_uri_ftp, "ftp://127.0.0.1", "127.0.0.1", "/",);

test_url_to_uri!(test_url_to_uri_username, "http://user@127.0.0.1/", "user@127.0.0.1", "/",);

test_url_to_uri!(test_url_to_uri_password, "http://:pass@127.0.0.1/", ":pass@127.0.0.1", "/",);

test_url_to_uri!(
	test_url_to_uri_username_password,
	"http://user:pass@127.0.0.1/",
	"user:pass@127.0.0.1",
	"/",
);

test_url_to_uri!(test_url_to_uri_username_port, "http://user@127.0.0.1:8080/", "user@127.0.0.1:8080", "/",);

test_url_to_uri!(test_url_to_uri_password_port, "http://:pass@127.0.0.1:8080/", ":pass@127.0.0.1:8080", "/",);

test_url_to_uri!(
	test_url_to_uri_username_password_port,
	"http://user:pass@127.0.0.1:8080/",
	"user:pass@127.0.0.1:8080",
	"/",
);

test_url_to_uri!(
	test_url_to_uri_query,
	"http://127.0.0.1/chunks?query=path",
	"127.0.0.1",
	"/chunks?query=path",
);

test_url_to_uri!(test_url_to_uri_fragment, "http://127.0.0.1/chunks#fragment", "127.0.0.1", "/chunks",);

test_url_to_uri!(
	test_url_to_uri_path_query_fragment,
	"http://127.0.0.1/chunks?query=path#fragment",
	"127.0.0.1",
	"/chunks?query=path",
);

test_url_to_uri!(
	test_url_to_uri_default_http_port,
	"http://127.0.0.1:80",
	"127.0.0.1",
	"/",
	"http://127.0.0.1",
);

test_url_to_uri!(
	test_url_to_uri_default_https_port,
	"https://127.0.0.1:443",
	"127.0.0.1",
	"/",
	"https://127.0.0.1",
);

test_url_to_uri_fail!(test_url_to_uri_unix_socket, "unix:/etc/socket.socket", Error::InvalidAuthority,);

test_url_to_uri_fail!(test_url_to_uri_data, "data:text/plain,Text", Error::InvalidAuthority,);

test_url_to_uri_fail!(test_url_to_uri_file_no_host, "file:/path", Error::InvalidHost,);
