use crate::executor::{default_executor, proxy_executor, Executor};
use crate::methods::Method;
use crate::poll::UpdatesStream;
use crate::types::Response;
use failure::Error;
use futures::{future, Future, Poll};
use serde::de::DeserializeOwned;
use std::rc::Rc;

/// Telegram Bot API client
#[derive(Clone)]
pub struct Api {
    executor: Rc<Box<Executor>>,
    token: String,
}

impl Api {
    /// Creates a client
    pub fn create<S: Into<String>>(token: S) -> Result<Self, Error> {
        Ok(Api {
            executor: Rc::new(default_executor()?),
            token: token.into(),
        })
    }

    /// Creates a client with specified proxy
    ///
    /// Proxy format:
    /// * http://[user:password]host:port
    /// * https://[user:password]@host:port
    /// * socks4://userid@host:port
    /// * socks5://[user:password]@host:port
    pub fn with_proxy<S: Into<String>>(token: S, url: &str) -> Result<Self, Error> {
        Ok(Api {
            executor: Rc::new(proxy_executor(url)?),
            token: token.into(),
        })
    }

    /// Executes a method
    pub fn execute<M: Method>(&self, method: &M) -> ApiFuture<M::Response>
    where
        M::Response: DeserializeOwned,
    {
        let executor = self.executor.clone();
        ApiFuture {
            inner: Box::new(
                future::result(
                    method
                        .get_request()
                        .map(|builder| builder.build(&self.token)),
                )
                .and_then(move |req| executor.execute(req).from_err())
                .and_then(|data| {
                    future::result(serde_json::from_slice::<Response<M::Response>>(&data))
                        .from_err()
                })
                .and_then(|rep| {
                    future::result(match rep {
                        Response::Success(obj) => Ok(obj),
                        Response::Error(err) => Err(err.into()),
                    })
                }),
            ),
        }
    }

    /// Returns an updates stream used for long polling
    pub fn get_updates(&self) -> UpdatesStream {
        UpdatesStream::new(self.clone())
    }
}

/// An API future
#[must_use = "futures do nothing unless polled"]
pub struct ApiFuture<T> {
    inner: Box<Future<Item = T, Error = Error>>,
}

impl<T> Future for ApiFuture<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}