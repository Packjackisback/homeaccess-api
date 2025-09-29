use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct CachedClient {
    pub client: Client,
    pub expires_at: Instant,
}

#[derive(Clone)]
pub struct CachedData {
    pub data: String,
    pub expires_at: Instant,
}

pub struct Cache {
    clients: Arc<RwLock<HashMap<String, CachedClient>>>,
    pages: Arc<RwLock<HashMap<String, CachedData>>>,
    client_ttl: Duration,
    page_ttl: Duration,
}

impl Cache {
    pub fn new(client_ttl_secs: u64, page_ttl_secs: u64) -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            pages: Arc::new(RwLock::new(HashMap::new())),
            client_ttl: Duration::from_secs(client_ttl_secs),
            page_ttl: Duration::from_secs(page_ttl_secs),
        }
    }

    fn make_client_key(username: &str, url: &str) -> String {
        format!("{}:{}", username, url)
    }

    fn make_page_key(username: &str, url: &str, endpoint: &str, params: &str) -> String {
        format!("{}:{}:{}:{}", username, url, endpoint, params)
    }

    pub async fn get_client(&self, username: &str, url: &str) -> Option<Client> {
        let key = Self::make_client_key(username, url);
        let clients = self.clients.read().await;
        
        if let Some(cached) = clients.get(&key) {
            if Instant::now() < cached.expires_at {
                return Some(cached.client.clone());
            }
        }
        None
    }

    pub async fn set_client(&self, username: &str, url: &str, client: Client) {
        let key = Self::make_client_key(username, url);
        let cached = CachedClient {
            client,
            expires_at: Instant::now() + self.client_ttl,
        };
        
        let mut clients = self.clients.write().await;
        clients.insert(key, cached);
    }

    pub async fn get_page(
        &self,
        username: &str,
        url: &str,
        endpoint: &str,
        params: &str,
    ) -> Option<String> {
        let key = Self::make_page_key(username, url, endpoint, params);
        let pages = self.pages.read().await;
        
        if let Some(cached) = pages.get(&key) {
            if Instant::now() < cached.expires_at {
                return Some(cached.data.clone());
            }
        }
        None
    }

    pub async fn set_page(
        &self,
        username: &str,
        url: &str,
        endpoint: &str,
        params: &str,
        data: String,
    ) {
        let key = Self::make_page_key(username, url, endpoint, params);
        let cached = CachedData {
            data,
            expires_at: Instant::now() + self.page_ttl,
        };
        
        let mut pages = self.pages.write().await;
        pages.insert(key, cached);
    }

    pub async fn clear_expired(&self) {
        let now = Instant::now();
        
        let mut clients = self.clients.write().await;
        clients.retain(|_, v| now < v.expires_at);
        
        let mut pages = self.pages.write().await;
        pages.retain(|_, v| now < v.expires_at);
    }
    
}

impl Clone for Cache {
    fn clone(&self) -> Self {
        Self {
            clients: Arc::clone(&self.clients),
            pages: Arc::clone(&self.pages),
            client_ttl: self.client_ttl,
            page_ttl: self.page_ttl,
        }
    }
}
