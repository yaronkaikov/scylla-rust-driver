use std::{sync::Arc, time::Duration};

use thiserror::Error;

use crate::transport::execution_profile::ExecutionProfileHandle;
use crate::{history::HistoryListener, retry_policy::RetryPolicy};

pub mod batch;
pub mod prepared_statement;
pub mod query;

pub use crate::frame::types::{Consistency, SerialConsistency};

pub use crate::frame::request::query::{PagingState, PagingStateResponse};

// This is the default common to drivers.
const DEFAULT_PAGE_SIZE: i32 = 5000;

#[derive(Debug, Clone, Default)]
pub(crate) struct StatementConfig {
    pub(crate) consistency: Option<Consistency>,
    pub(crate) serial_consistency: Option<Option<SerialConsistency>>,

    pub(crate) is_idempotent: bool,

    pub(crate) skip_result_metadata: bool,
    pub(crate) tracing: bool,
    pub(crate) timestamp: Option<i64>,
    pub(crate) request_timeout: Option<Duration>,

    pub(crate) history_listener: Option<Arc<dyn HistoryListener>>,

    pub(crate) execution_profile_handle: Option<ExecutionProfileHandle>,
    pub(crate) retry_policy: Option<Arc<dyn RetryPolicy>>,
}

impl StatementConfig {
    /// Determines the consistency of a query
    #[must_use]
    pub(crate) fn determine_consistency(&self, default_consistency: Consistency) -> Consistency {
        self.consistency.unwrap_or(default_consistency)
    }
}

#[derive(Debug, Clone, Copy, Error)]
#[error("Invalid page size provided: {0}; valid values are [1, i32::MAX]")]
/// Invalid page size was provided.
pub(crate) struct InvalidPageSize(i32);

/// Size of a single page when performing paged queries to the DB.
/// Configurable on statements, used in `Session::{query,execute}_{iter,single_page}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PageSize(i32);

impl PageSize {
    /// Creates a new positive page size. If a non-positive number is passed,
    /// returns an [InvalidPageSize] error.
    #[inline]
    pub(crate) fn new(size: i32) -> Result<Self, InvalidPageSize> {
        if size > 0 {
            Ok(Self(size))
        } else {
            Err(InvalidPageSize(size))
        }
    }

    #[inline]
    pub(crate) fn inner(&self) -> i32 {
        self.0
    }
}

impl Default for PageSize {
    #[inline]
    fn default() -> Self {
        Self(DEFAULT_PAGE_SIZE)
    }
}

impl TryFrom<i32> for PageSize {
    type Error = InvalidPageSize;

    #[inline]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PageSize> for i32 {
    #[inline]
    fn from(page_size: PageSize) -> Self {
        page_size.inner()
    }
}
