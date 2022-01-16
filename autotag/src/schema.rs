#![allow(nonstandard_style)]

use std::collections::{HashSet, HashMap};

use serde::{Serialize, Deserialize};
use strum_macros::{Display};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Category {
    General,
    Character,
    Copyright,
    Author,
    Meta,
    Language,
    Soundtrack,
}


#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
)]
#[repr(u32)]
pub enum Languages {
    NAL,
    CHS,
    CHT,
    CSY,
    NLD,
    ENG,
    FRA,
    DEU,
    HUN,
    ITA,
    JPN,
    KOR,
    PLK,
    PTB,
    ROM,
    RUS,
    ESP,
    TRK,
    VIN,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    pub id: u32,
    pub category: Category,
    pub count: u32,
    pub icon: String,
    pub alias: HashSet<String>,
    pub languages: HashMap<Languages, String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct V1InferTagRequest {
    /// Video title
    pub title: String,
    /// Video description
    pub desc: String,
    /// Video tags provided by its source website
    pub utags: HashSet<String>,
    /// Video thumbnail URL, maybe used for image recognition later, not currently in use
    pub thumb_url: Option<String>,
    /// Return tags with confidence above this threshold, range [0, 1], not currently in use
    pub thres: Option<f32>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct V1InferTagResponse {
    /// A set of tag IDs
    pub tagids: HashSet<u32>
}
