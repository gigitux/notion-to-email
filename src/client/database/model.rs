use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseResponse {
    pub object: String,
    pub results: Vec<Result>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub object: String,
    pub id: String,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "last_edited_time")]
    pub last_edited_time: String,
    #[serde(rename = "created_by")]
    pub created_by: CreatedBy,
    #[serde(rename = "last_edited_by")]
    pub last_edited_by: LastEditedBy,
    pub cover: Value,
    pub icon: Value,
    pub parent: Parent,
    pub archived: bool,
    #[serde(rename = "in_trash")]
    pub in_trash: bool,
    pub properties: Properties,
    pub url: String,
    #[serde(rename = "public_url")]
    pub public_url: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedBy {
    pub object: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastEditedBy {
    pub object: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "database_id")]
    pub database_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    #[serde(rename = "URL")]
    pub url: Url,
    #[serde(rename = "Name")]
    pub name: Name,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: Vec<Title>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Text,
    pub annotations: Annotations,
    #[serde(rename = "plain_text")]
    pub plain_text: String,
    pub href: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub content: String,
    pub link: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: String,
}
