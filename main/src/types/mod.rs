pub mod github;

use std::sync::Arc;
use twilight_http::Client as HttpClient;

pub type TwHttpClient = Arc<HttpClient>;
