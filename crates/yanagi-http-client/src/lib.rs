use serde::de::DeserializeOwned;
use std::io::Read;
use std::{thread, time::Duration};

#[derive(thiserror::Error, Debug)]
pub enum FetchJsonError {
    #[error("http error: {0}")]
    Http(#[from] ureq::Error),
    #[error("deserialization error: {0}")]
    Deserialize(String),
}

pub struct AutopatchClient<'url, T> {
    base_url: &'url str,
    agent: ureq::Agent,
    retry_after: T,
}

impl<'url> AutopatchClient<'url, ()> {
    pub fn new(base_url: &'url str) -> Self {
        Self {
            base_url,
            agent: ureq::AgentBuilder::new().build(),
            retry_after: (),
        }
    }

    pub fn retry_after(self, retry_after: Duration) -> AutopatchClient<'url, Duration> {
        AutopatchClient {
            retry_after,
            agent: self.agent,
            base_url: self.base_url,
        }
    }
}

impl AutopatchClient<'_, Duration> {
    pub fn fetch_until_success<T: DeserializeOwned>(&self, url: &str) -> T {
        loop {
            match self.fetch_json(url) {
                Ok(data) => break data,
                Err(FetchJsonError::Http(_)) => thread::sleep(self.retry_after),
                Err(FetchJsonError::Deserialize(err)) => panic!("failed to deserlize {url}: {err}"),
            }
        }
    }

    pub fn fetch_bytes_until_success(&self, url: &str) -> Box<[u8]> {
        loop {
            match self.fetch_bytes(url) {
                Ok(data) => break data,
                Err(_) => thread::sleep(self.retry_after),
            }
        }
    }
}

impl<R> AutopatchClient<'_, R> {
    pub fn fetch(&self, path: &str) -> Result<String, ureq::Error> {
        let result = match path {
            url if path.starts_with("http://") || path.starts_with("https://") => {
                self.agent.get(url).call()?.into_string()?
            }
            path => self
                .agent
                .get(format!("{}{}", self.base_url, path).as_str())
                .call()?
                .into_string()?,
        };

        Ok(result)
    }

    pub fn fetch_bytes(&self, path: &str) -> Result<Box<[u8]>, ureq::Error> {
        let mut result = match path {
            url if path.starts_with("http://") || path.starts_with("https://") => {
                self.agent.get(url).call()?.into_reader()
            }
            path => self
                .agent
                .get(format!("{}{}", self.base_url, path).as_str())
                .call()?
                .into_reader(),
        };

        let mut buf = Vec::new();
        result.read_to_end(&mut buf)?;
        Ok(buf.into_boxed_slice())
    }

    pub fn fetch_json<T: DeserializeOwned>(&self, url: &str) -> Result<T, FetchJsonError> {
        let content = self.fetch(url)?;
        serde_json::from_str(content.as_str())
            .map_err(|err| FetchJsonError::Deserialize(err.to_string()))
    }
}
