/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingCloudfiles {
    /// The name for the real-time logging configuration.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`. 
    #[serde(rename = "placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<Placement>,
    /// The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`. 
    #[serde(rename = "format_version", skip_serializing_if = "Option::is_none")]
    pub format_version: Option<FormatVersion>,
    /// The name of an existing condition in the configured endpoint, or leave blank to always execute.
    #[serde(rename = "response_condition", skip_serializing_if = "Option::is_none")]
    pub response_condition: Option<String>,
    /// A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats).
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// How the message should be formatted.
    #[serde(rename = "message_type", skip_serializing_if = "Option::is_none")]
    pub message_type: Option<MessageType>,
    /// A timestamp format
    #[serde(rename = "timestamp_format", skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<String>,
    /// How frequently log files are finalized so they can be available for reading (in seconds).
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    /// The level of gzip encoding when sending logs (default `0`, no compression). Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error.
    #[serde(rename = "gzip_level", skip_serializing_if = "Option::is_none")]
    pub gzip_level: Option<i32>,
    /// The codec used for compressing your logs. Valid values are `zstd`, `snappy`, and `gzip`. Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error.
    #[serde(rename = "compression_codec", skip_serializing_if = "Option::is_none")]
    pub compression_codec: Option<CompressionCodec>,
    /// Your Cloud Files account access key.
    #[serde(rename = "access_key", skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    /// The name of your Cloud Files container.
    #[serde(rename = "bucket_name", skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// The path to upload logs to.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The region to stream logs to.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<Region>,
    /// A PGP public key that Fastly will use to encrypt your log files before writing them to disk.
    #[serde(rename = "public_key", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// The username for your Cloud Files account.
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl LoggingCloudfiles {
    pub fn new() -> LoggingCloudfiles {
        LoggingCloudfiles {
            name: None,
            placement: None,
            format_version: None,
            response_condition: None,
            format: None,
            message_type: None,
            timestamp_format: None,
            period: None,
            gzip_level: None,
            compression_codec: None,
            access_key: None,
            bucket_name: None,
            path: None,
            region: None,
            public_key: None,
            user: None,
        }
    }
}

/// Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Placement {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "waf_debug")]
    WafDebug,
    #[serde(rename = "null")]
    Null,
}

impl Default for Placement {
    fn default() -> Placement {
        Self::None
    }
}
/// The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FormatVersion {
    #[serde(rename = "1")]
    FormatVersionV1,
    #[serde(rename = "2")]
    FormatVersionV2,
}

impl Default for FormatVersion {
    fn default() -> FormatVersion {
        Self::FormatVersionV1
    }
}
/// How the message should be formatted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MessageType {
    #[serde(rename = "classic")]
    Classic,
    #[serde(rename = "loggly")]
    Loggly,
    #[serde(rename = "logplex")]
    Logplex,
    #[serde(rename = "blank")]
    Blank,
}

impl Default for MessageType {
    fn default() -> MessageType {
        Self::Classic
    }
}
/// The codec used for compressing your logs. Valid values are `zstd`, `snappy`, and `gzip`. Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CompressionCodec {
    #[serde(rename = "zstd")]
    Zstd,
    #[serde(rename = "snappy")]
    Snappy,
    #[serde(rename = "gzip")]
    Gzip,
}

impl Default for CompressionCodec {
    fn default() -> CompressionCodec {
        Self::Zstd
    }
}
/// The region to stream logs to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Region {
    #[serde(rename = "DFW")]
    DFW,
    #[serde(rename = "ORD")]
    ORD,
    #[serde(rename = "IAD")]
    IAD,
    #[serde(rename = "LON")]
    LON,
    #[serde(rename = "SYD")]
    SYD,
    #[serde(rename = "HKG")]
    HKG,
    #[serde(rename = "null")]
    Null,
}

impl Default for Region {
    fn default() -> Region {
        Self::DFW
    }
}

