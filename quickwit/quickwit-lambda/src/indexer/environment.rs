// Copyright (C) 2024 Quickwit, Inc.
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

use std::env::var;

use once_cell::sync::Lazy;

pub const CONFIGURATION_TEMPLATE: &str = "version: 0.7
node_id: lambda-indexer
cluster_id: lambda-ephemeral
metastore_uri: s3://${QW_LAMBDA_METASTORE_BUCKET}/index
default_index_root_uri: s3://${QW_LAMBDA_INDEX_BUCKET}/index
data_dir: /tmp
";

pub static INDEX_CONFIG_URI: Lazy<String> = Lazy::new(|| {
    var("QW_LAMBDA_INDEX_CONFIG_URI").expect("QW_LAMBDA_INDEX_CONFIG_URI must be set")
});

pub static DISABLE_MERGE: Lazy<bool> =
    Lazy::new(|| var("QW_LAMBDA_DISABLE_MERGE").is_ok_and(|v| v.as_str() == "true"));

pub static DISABLE_JANITOR: Lazy<bool> =
    Lazy::new(|| var("QW_LAMBDA_DISABLE_JANITOR").is_ok_and(|v| v.as_str() == "true"));

pub static MAX_CHECKPOINTS: Lazy<usize> = Lazy::new(|| {
    var("QW_LAMBDA_MAX_CHECKPOINTS").map_or(100, |v| {
        v.parse()
            .expect("QW_LAMBDA_MAX_CHECKPOINTS must be a positive integer")
    })
});
