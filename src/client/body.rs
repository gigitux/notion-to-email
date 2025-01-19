use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct NotionBody {
    pub parent: Option<Parent>,
    pub properties: HashMap<String, PropertyValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parent {
    pub database_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PropertyValue {
    Title { title: Vec<TextContent> },
    RichText { rich_text: Vec<TextContent> },
    Number { number: f64 },
    Select { select: Option<SelectOption> },
    Checkbox { checkbox: bool },
    Date { date: Option<DateValue> },
    MultiSelect { multi_select: Vec<SelectOption> },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextContent {
    pub text: Text,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectOption {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DateValue {
    pub start: String,
    pub end: Option<String>,
}
