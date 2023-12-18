// Copyright (C) 2023 Quickwit, Inc.
//
// Quickwit is offered under the AGPL v3.0 and as commercial software.
// For commercial licensing, contact us at hello@quickwit.io.
//
// AGPL:
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

mod bulk_body;
mod bulk_query_params;
mod error;
mod field_capability;
mod multi_search;
mod scroll;
mod search_body;
mod search_query_params;

pub use bulk_body::{BulkAction, BulkActionMeta};
pub use bulk_query_params::{ElasticIngestOptions, ElasticRefresh};
pub use error::ElasticSearchError;
pub use field_capability::{
    build_list_field_request_for_es_api, convert_to_es_field_capabilities_response,
    FieldCapabilityQueryParams, FieldCapabilityRequestBody, FieldCapabilityResponse,
};
pub use multi_search::{
    MultiSearchHeader, MultiSearchQueryParams, MultiSearchResponse, MultiSearchSingleResponse,
};
use quickwit_proto::search::{SortDatetimeFormat, SortOrder};
pub use scroll::ScrollQueryParams;
pub use search_body::SearchBody;
pub use search_query_params::SearchQueryParams;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SortField {
    pub field: String,
    pub order: SortOrder,
    pub date_format: Option<ElasticDateFormat>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ElasticDateFormat {
    /// We don't want to use the format `EpochMillis` as elasticsearch
    /// returns milliseconds as strings when used. Instead, we support
    /// `EpochMillisAsInt` which returns milliseconds as integers to
    /// make it explicit for the user.
    EpochMillisAsInt,
}

impl From<ElasticDateFormat> for SortDatetimeFormat {
    fn from(date_format: ElasticDateFormat) -> Self {
        match date_format {
            ElasticDateFormat::EpochMillisAsInt => SortDatetimeFormat::UnixTimestampMillis,
        }
    }
}

pub(crate) fn default_elasticsearch_sort_order(field_name: &str) -> SortOrder {
    if field_name == "_score" {
        SortOrder::Desc
    } else {
        SortOrder::Asc
    }
}
