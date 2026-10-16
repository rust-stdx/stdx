//! List responses and automatic pagination.

use core::marker::PhantomData;

use serde::de::DeserializeOwned;

use crate::{client::Client, error::Error};

/// A single page of a Stripe list response.
///
/// Stripe list endpoints return an object of this shape. Use [`Pager`] (built
/// with [`Client::paginate`](crate::Client::paginate)) to walk every page without
/// hand-managing cursors.
///
/// v1 lists carry `has_more` and are followed with `starting_after`; v2 lists
/// carry `next_page_url`, which [`Pager`] follows after checking that it stays
/// on the configured host.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct List<T> {
    /// The object type, always `"list"`.
    #[serde(default)]
    pub object: String,
    /// The items on this page.
    pub data: Vec<T>,
    /// Whether more pages are available (v1 lists).
    #[serde(default)]
    pub has_more: bool,
    /// The URL this list was fetched from.
    #[serde(default)]
    pub url: String,
    /// Total count, when the endpoint provides one.
    #[serde(default)]
    pub total_count: Option<u64>,
    /// The URL of the next page, for the v2 APIs that page by URL.
    #[serde(default)]
    pub next_page_url: Option<String>,
}

impl<T> List<T> {
    /// Returns true when another page can be fetched.
    pub fn has_next_page(&self) -> bool {
        self.has_more || self.next_page_url.as_deref().is_some_and(|url| !url.is_empty())
    }
}

/// A resource that carries an `id`, which is what pagination cursors need.
///
/// Implemented by the generated resource types.
pub trait PagedItem {
    /// Returns the object's `id`.
    fn id(&self) -> &str;
}

/// Iterates over the pages of a Stripe list endpoint.
///
/// v1 lists are followed with a `starting_after` cursor; v2 lists are followed
/// through their `next_page_url`. The pager guards against a server that reports
/// more pages but never advances the cursor or the URL, which would otherwise
/// loop forever, and refuses to follow a `next_page_url` that leaves the
/// configured base URL.
pub struct Pager<'a, T> {
    client: &'a Client,
    path: String,
    base_query: String,
    cursor: Option<String>,
    last_cursor: Option<String>,
    /// The v2 `next_page_url` of the previous page, if it provided one.
    next_url: Option<String>,
    last_url: Option<String>,
    has_more: bool,
    finished: bool,
    marker: PhantomData<fn() -> T>,
}

impl<'a, T> Pager<'a, T> {
    /// Creates a pager that will fetch the first page on the first call to
    /// [`Pager::next_page`].
    pub fn new(client: &'a Client, path: impl Into<String>, base_query: impl Into<String>) -> Self {
        Self {
            client,
            path: path.into(),
            base_query: base_query.into(),
            cursor: None,
            last_cursor: None,
            next_url: None,
            last_url: None,
            has_more: true,
            finished: false,
            marker: PhantomData,
        }
    }

    /// Skips ahead to the page after `cursor`.
    pub fn starting_after(mut self, cursor: impl Into<String>) -> Self {
        let cursor = cursor.into();
        self.cursor = Some(cursor.clone());
        self.last_cursor = Some(cursor);
        self
    }

    /// Fetches the next page, or returns `None` when the list is exhausted.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails, if the response cannot be decoded,
    /// if the server reports more pages without advancing the cursor (which is
    /// treated as a protocol violation rather than looping forever), or if a v2
    /// `next_page_url` points outside the configured base URL.
    pub async fn next_page(&mut self) -> Result<Option<List<T>>, Error>
    where
        T: DeserializeOwned + PagedItem,
    {
        if self.finished || !self.has_more {
            return Ok(None);
        }

        // v2 lists page by URL; v1 lists page by cursor.
        let page: List<T> = match self.next_url.clone() {
            Some(url) => self.client.get_raw_url(&url).await?,
            None => {
                let query = match &self.cursor {
                    Some(cursor) => {
                        let mut query = String::with_capacity(self.base_query.len() + cursor.len() + 20);
                        query.push_str(&self.base_query);
                        if !query.is_empty() {
                            query.push('&');
                        }
                        query.push_str("starting_after=");
                        crate::params::percent_encode_into(&mut query, cursor);
                        query
                    }
                    None => self.base_query.clone(),
                };
                self.client.get_raw(&self.path, &query).await?
            }
        };

        let next_url = page.next_page_url.clone().filter(|url| !url.is_empty());
        if let Some(url) = &next_url {
            if self.last_url.as_deref() == Some(url.as_str()) {
                return Err(Error::InvalidRequest(
                    "pagination next_page_url did not advance; aborting to avoid an infinite loop".into(),
                ));
            }
            self.last_url = Some(url.clone());
        }
        self.next_url = next_url;

        self.has_more = page.has_more || self.next_url.is_some();
        if !self.has_more {
            self.finished = true;
        }

        if self.next_url.is_none() {
            self.cursor = match page.data.last() {
                Some(last) => {
                    let next = last.id().to_string();
                    if self.last_cursor.as_deref() == Some(next.as_str()) {
                        // The cursor did not advance: stop instead of looping.
                        return Err(Error::InvalidRequest(
                            "pagination cursor did not advance; aborting to avoid an infinite loop".into(),
                        ));
                    }
                    self.last_cursor = Some(next.clone());
                    Some(next)
                }
                None => {
                    if page.has_more {
                        return Err(Error::InvalidRequest("pagination reported has_more with an empty page".into()));
                    }
                    None
                }
            };
        }

        Ok(Some(page))
    }

    /// Returns true when no further pages will be fetched.
    pub fn is_finished(&self) -> bool {
        self.finished || !self.has_more
    }
}
