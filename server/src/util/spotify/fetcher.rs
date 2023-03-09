use std::marker::PhantomData;

use rspotify::http::HttpError;
use rspotify::model::{AlbumId, FullAlbum, SearchResult, SearchType};
use rspotify::prelude::BaseClient;
use rspotify::{AuthCodeSpotify, ClientError, ClientResult};

use crate::error::{PhonosError, PhonosResult};

pub enum FetcherType {
    Search {
        query: String,
        search_type: SearchType,
    },
    Album {
        album_id: String,
    },
}

pub struct Fetcher<T> {
    fetcher_type: FetcherType,
    phantom: PhantomData<T>,
}

impl<T> Fetcher<T> {
    pub fn new(t: FetcherType) -> Self {
        Self {
            fetcher_type: t,
            phantom: PhantomData,
        }
    }
}

impl Fetcher<SearchResult> {
    pub async fn execute(&self, client: &AuthCodeSpotify) -> PhonosResult<SearchResult> {
        match self.fetcher_type {
            FetcherType::Search {
                ref query,
                search_type,
            } => {
                let result = client
                    .search(query, search_type, None, None, None, None)
                    .await;

                if is_access_error(&result) {
                    println!("Refreshing access token");
                    client.refresh_token().await?;

                    Ok(client
                        .search(query, search_type, None, None, None, None)
                        .await?)
                } else {
                    Ok(result?)
                }
            }
            _ => Err(PhonosError::OtherError("Invalid fetcher type".to_string())),
        }
    }
}

impl Fetcher<FullAlbum> {
    pub async fn execute(&self, client: &AuthCodeSpotify) -> PhonosResult<FullAlbum> {
        match self.fetcher_type {
            FetcherType::Album { ref album_id } => {
                let album_id = AlbumId::from_id(album_id)?;
                let result = client.album(album_id.clone()).await;

                if is_access_error(&result) {
                    println!("Refreshing access token");
                    client.refresh_token().await?;

                    Ok(client.album(album_id).await?)
                } else {
                    Ok(result?)
                }
            }
            _ => Err(PhonosError::OtherError("Invalid fetcher type".to_string())),
        }
    }
}

fn is_access_error<T>(result: &ClientResult<T>) -> bool {
    if let Err(ClientError::Http(ref e)) = result {
        if let HttpError::StatusCode(e) = e.as_ref() {
            return e.status().as_u16() == 401;
        }
    }
    false
}
