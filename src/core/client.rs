use crate::core::error::CoreError;
use chrono::{DateTime, Utc};
use log;
use reqwest;
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::result::Result;
use url::Url;

// hand crafted struct mirror helm go pkg,
// https://github.com/helm/helm/blob/efe2638f87b597403f34009e1029a6f0b44db8f2/pkg/chart/metadata.go
#[derive(Deserialize, Debug)]
pub struct MetaData {
    pub name: String,
    pub version: String,
    pub created: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct Index(HashMap<String, MetaData>);

pub struct Client {
    base_url: url::Url,
    basic_auth: Option<String>,
}

impl Client {
    pub fn new(base_url: &str, _basic_auth: Option<&str>) -> Result<Self, CoreError> {
        log::debug!("Initing client...");
        let base = Url::parse(base_url)?;
        log::debug!("base url parsed as {}", base.as_str());
        if !(base.scheme() == "http" || base.scheme() == "https") {
            return Err(CoreError::EndpointConfigError(format!(
                "Invalid scheme: {}",
                base.scheme()
            )));
        }
        Ok(Self {
            base_url: base,
            basic_auth: None,
        })
    }

    fn index_url(&self) -> String {
        self.base_url
            .join("api/charts")
            .unwrap()
            .as_str()
            .to_string()
    }

    fn chart_url(&self, chart_name: &str) -> String {
        self.base_url
            .join(&format!("api/charts/{}", chart_name))
            .unwrap()
            .as_str()
            .to_string()
    }

    fn delete_url(&self, chart_name: &str, version: &str) -> String {
        self.base_url
            .join(&format!("api/charts/{}/{}", chart_name, version))
            .unwrap()
            .as_str()
            .to_string()
    }

    pub fn list_charts(&self) -> std::result::Result<Vec<String>, CoreError> {
        let url = self.index_url();
        log::debug!("listing repos use url: [{}]", &url);
        let resp = reqwest::blocking::get(url)?.error_for_status()?.text()?;
        log::trace!("list repos response: {}", &resp);
        let repos_map: HashMap<String, serde_json::Value> = serde_json::from_str(&resp)?;
        log::debug!("success deserialize repo names vec from response");
        Ok(repos_map.keys().cloned().collect())
    }

    pub fn chart_versions(
        &self,
        chart_name: &str,
    ) -> std::result::Result<Vec<MetaData>, CoreError> {
        let url = self.chart_url(chart_name);
        log::debug!("get versions use url: [{}]", &url);
        let resp = reqwest::blocking::get(url)?.text()?;
        log::trace!("get versions response: {}", &resp);
        let metadata: Vec<MetaData> = serde_json::from_str(&resp)?;
        log::debug!("success deserialize versions vec from response");
        Ok(metadata)
    }

    pub fn del_chart_version(
        &self,
        chart_name: &str,
        chart_version: &str,
    ) -> std::result::Result<(), CoreError> {
        let url = self.delete_url(chart_name, chart_version);
        log::debug!("delete version use url: [{}]", &url);
        let client = reqwest::blocking::Client::new();
        let _resp = client.delete(url).send()?;
        log::debug!("success delete version {}-{}", chart_name, chart_version);
        Ok(())
    }
}
