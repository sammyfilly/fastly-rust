/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Pop {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "coordinates", skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<Box<crate::models::PopCoordinates>>,
    #[serde(rename = "shield", skip_serializing_if = "Option::is_none")]
    pub shield: Option<String>,
}

impl Pop {
    pub fn new() -> Pop {
        Pop {
            code: None,
            name: None,
            group: None,
            coordinates: None,
            shield: None,
        }
    }
}


