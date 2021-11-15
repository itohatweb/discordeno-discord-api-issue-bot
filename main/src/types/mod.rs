pub mod github;

use std::sync::Arc;
use twilight_cache_inmemory::InMemoryCache;
use twilight_http::Client as HttpClient;

pub type TwHttpClient = Arc<HttpClient>;

pub type Acache = std::sync::Arc<InMemoryCache>;
