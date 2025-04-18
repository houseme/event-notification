use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::collections::HashMap;
use strum::{Display, EnumString};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Identity {
    #[serde(rename = "principalId")]
    pub principal_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bucket {
    pub name: String,
    #[serde(rename = "ownerIdentity")]
    pub owner_identity: Identity,
    pub arn: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Object {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eTag")]
    pub etag: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contentType"
    )]
    pub content_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "userMetadata"
    )]
    pub user_metadata: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "versionId")]
    pub version_id: Option<String>,
    pub sequencer: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Metadata {
    #[serde(rename = "s3SchemaVersion")]
    pub schema_version: String,
    #[serde(rename = "configurationId")]
    pub configuration_id: String,
    pub bucket: Bucket,
    pub object: Object,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Source {
    pub host: String,
    pub port: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    #[serde(rename = "eventVersion")]
    pub event_version: String,
    #[serde(rename = "eventSource")]
    pub event_source: String,
    #[serde(rename = "awsRegion")]
    pub aws_region: String,
    #[serde(rename = "eventTime")]
    pub event_time: String,
    #[serde(rename = "eventName")]
    pub event_name: Name,
    #[serde(rename = "userIdentity")]
    pub user_identity: Identity,
    #[serde(rename = "requestParameters")]
    pub request_parameters: HashMap<String, String>,
    #[serde(rename = "responseElements")]
    pub response_elements: HashMap<String, String>,
    pub s3: Metadata,
    pub source: Source,
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub channels: Vec<String>,
}

impl Event {
    pub fn new(
        event_version: &str,
        event_source: &str,
        aws_region: &str,
        event_time: &str,
        event_name: Name,
        user_identity: Identity,
        request_parameters: HashMap<String, String>,
        response_elements: HashMap<String, String>,
        s3: Metadata,
        source: Source,
        channels: Vec<String>,
    ) -> Self {
        Self {
            event_version: event_version.to_string(),
            event_source: event_source.to_string(),
            aws_region: aws_region.to_string(),
            event_time: event_time.to_string(),
            event_name,
            user_identity,
            request_parameters,
            response_elements,
            s3,
            source,
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            channels,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Log {
    #[serde(rename = "eventName")]
    pub event_name: Name,
    pub key: String,
    pub records: Vec<Event>,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, Display, EnumString,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Name {
    ObjectAccessedGet,
    ObjectAccessedGetRetention,
    ObjectAccessedGetLegalHold,
    ObjectAccessedHead,
    ObjectAccessedAttributes,
    ObjectCreatedCompleteMultipartUpload,
    ObjectCreatedCopy,
    ObjectCreatedPost,
    ObjectCreatedPut,
    ObjectCreatedPutRetention,
    ObjectCreatedPutLegalHold,
    ObjectCreatedPutTagging,
    ObjectCreatedDeleteTagging,
    ObjectRemovedDelete,
    ObjectRemovedDeleteMarkerCreated,
    ObjectRemovedDeleteAllVersions,
    ObjectRemovedNoOp,
    BucketCreated,
    BucketRemoved,
    ObjectReplicationFailed,
    ObjectReplicationComplete,
    ObjectReplicationMissedThreshold,
    ObjectReplicationReplicatedAfterThreshold,
    ObjectReplicationNotTracked,
    ObjectRestorePost,
    ObjectRestoreCompleted,
    ObjectTransitionFailed,
    ObjectTransitionComplete,
    ObjectManyVersions,
    ObjectLargeVersions,
    PrefixManyFolders,
    IlmDelMarkerExpirationDelete,
    ObjectAccessedAll,
    ObjectCreatedAll,
    ObjectRemovedAll,
    ObjectReplicationAll,
    ObjectRestoreAll,
    ObjectTransitionAll,
    ObjectScannerAll,
    Everything,
}

impl Name {
    pub fn expand(&self) -> Vec<Name> {
        match self {
            Name::ObjectAccessedAll => vec![
                Name::ObjectAccessedGet,
                Name::ObjectAccessedHead,
                Name::ObjectAccessedGetRetention,
                Name::ObjectAccessedGetLegalHold,
                Name::ObjectAccessedAttributes,
            ],
            Name::ObjectCreatedAll => vec![
                Name::ObjectCreatedCompleteMultipartUpload,
                Name::ObjectCreatedCopy,
                Name::ObjectCreatedPost,
                Name::ObjectCreatedPut,
                Name::ObjectCreatedPutRetention,
                Name::ObjectCreatedPutLegalHold,
                Name::ObjectCreatedPutTagging,
                Name::ObjectCreatedDeleteTagging,
            ],
            Name::ObjectRemovedAll => vec![
                Name::ObjectRemovedDelete,
                Name::ObjectRemovedDeleteMarkerCreated,
                Name::ObjectRemovedNoOp,
                Name::ObjectRemovedDeleteAllVersions,
            ],
            Name::ObjectReplicationAll => vec![
                Name::ObjectReplicationFailed,
                Name::ObjectReplicationComplete,
                Name::ObjectReplicationNotTracked,
                Name::ObjectReplicationMissedThreshold,
                Name::ObjectReplicationReplicatedAfterThreshold,
            ],
            Name::ObjectRestoreAll => vec![Name::ObjectRestorePost, Name::ObjectRestoreCompleted],
            Name::ObjectTransitionAll => {
                vec![Name::ObjectTransitionFailed, Name::ObjectTransitionComplete]
            }
            Name::ObjectScannerAll => vec![
                Name::ObjectManyVersions,
                Name::ObjectLargeVersions,
                Name::PrefixManyFolders,
            ],
            Name::Everything => (1..=Name::IlmDelMarkerExpirationDelete as u32)
                .map(|i| Name::from_repr(i).unwrap())
                .collect(),
            _ => vec![*self],
        }
    }

    pub fn mask(&self) -> u64 {
        if (*self as u32) < Name::ObjectAccessedAll as u32 {
            1 << (*self as u32 - 1)
        } else {
            self.expand()
                .iter()
                .fold(0, |acc, n| acc | (1 << (*n as u32 - 1)))
        }
    }

    fn from_repr(discriminant: u32) -> Option<Self> {
        match discriminant {
            1 => Some(Name::ObjectAccessedGet),
            2 => Some(Name::ObjectAccessedGetRetention),
            3 => Some(Name::ObjectAccessedGetLegalHold),
            4 => Some(Name::ObjectAccessedHead),
            5 => Some(Name::ObjectAccessedAttributes),
            6 => Some(Name::ObjectCreatedCompleteMultipartUpload),
            7 => Some(Name::ObjectCreatedCopy),
            8 => Some(Name::ObjectCreatedPost),
            9 => Some(Name::ObjectCreatedPut),
            10 => Some(Name::ObjectCreatedPutRetention),
            11 => Some(Name::ObjectCreatedPutLegalHold),
            12 => Some(Name::ObjectCreatedPutTagging),
            13 => Some(Name::ObjectCreatedDeleteTagging),
            14 => Some(Name::ObjectRemovedDelete),
            15 => Some(Name::ObjectRemovedDeleteMarkerCreated),
            16 => Some(Name::ObjectRemovedDeleteAllVersions),
            17 => Some(Name::ObjectRemovedNoOp),
            18 => Some(Name::BucketCreated),
            19 => Some(Name::BucketRemoved),
            20 => Some(Name::ObjectReplicationFailed),
            21 => Some(Name::ObjectReplicationComplete),
            22 => Some(Name::ObjectReplicationMissedThreshold),
            23 => Some(Name::ObjectReplicationReplicatedAfterThreshold),
            24 => Some(Name::ObjectReplicationNotTracked),
            25 => Some(Name::ObjectRestorePost),
            26 => Some(Name::ObjectRestoreCompleted),
            27 => Some(Name::ObjectTransitionFailed),
            28 => Some(Name::ObjectTransitionComplete),
            29 => Some(Name::ObjectManyVersions),
            30 => Some(Name::ObjectLargeVersions),
            31 => Some(Name::PrefixManyFolders),
            32 => Some(Name::IlmDelMarkerExpirationDelete),
            33 => Some(Name::ObjectAccessedAll),
            34 => Some(Name::ObjectCreatedAll),
            35 => Some(Name::ObjectRemovedAll),
            36 => Some(Name::ObjectReplicationAll),
            37 => Some(Name::ObjectRestoreAll),
            38 => Some(Name::ObjectTransitionAll),
            39 => Some(Name::ObjectScannerAll),
            40 => Some(Name::Everything),
            _ => None,
        }
    }
}
