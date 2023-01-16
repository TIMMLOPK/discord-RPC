#![allow(dead_code)]
use serde::Serialize;

#[derive(Serialize)]
pub struct Activity<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamps: Option<Timestamps>,

    #[serde(skip_serializing_if = "Option::is_none")]
    assets: Option<Assets<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    buttons: Option<Vec<Buttons<'a>>>,
}

#[derive(Serialize)]
pub struct Timestamps {
    start: Option<i64>,
    end: Option<i64>,
}

#[derive(Serialize)]
pub struct Assets<'a> {
    large_image: Option<&'a str>,
    large_text: Option<&'a str>,
    small_image: Option<&'a str>,
    small_text: Option<&'a str>,
}

#[derive(Serialize)]
pub struct Buttons<'a> {
    label: Option<&'a str>,
    url: Option<&'a str>,
}

impl<'a> Activity<'a> {
    pub fn new() -> Self {
        Self {
            state: None,
            details: None,
            timestamps: None,
            assets: None,
            buttons: None,
        }
    }

    pub fn state(mut self, state: &'a str) -> Self {
        self.state = Some(state);
        self
    }

    pub fn details(mut self, details: &'a str) -> Self {
        self.details = Some(details);
        self
    }

    pub fn timestamps(mut self, timestamps: Timestamps) -> Self {
        self.timestamps = Some(timestamps);
        self
    }

    pub fn assets(mut self, assets: Assets<'a>) -> Self {
        self.assets = Some(assets);
        self
    }

    pub fn buttons(mut self, buttons: Vec<Buttons<'a>>) -> Self {
        self.buttons = Some(buttons);
        self
    }
}

impl<'a> Default for Activity<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl Timestamps {
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    pub fn start(mut self, start: i64) -> Self {
        self.start = Some(start);
        self
    }

    pub fn end(mut self, end: i64) -> Self {
        self.end = Some(end);
        self
    }
}

impl<'a> Default for Timestamps {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Assets<'a> {
    pub fn new() -> Self {
        Self {
            large_image: None,
            large_text: None,
            small_image: None,
            small_text: None,
        }
    }

    pub fn large_image(mut self, large_image: &'a str) -> Self {
        self.large_image = Some(large_image);
        self
    }

    pub fn large_text(mut self, large_text: &'a str) -> Self {
        self.large_text = Some(large_text);
        self
    }

    pub fn small_image(mut self, small_image: &'a str) -> Self {
        self.small_image = Some(small_image);
        self
    }

    pub fn small_text(mut self, small_text: &'a str) -> Self {
        self.small_text = Some(small_text);
        self
    }
}

impl<'a> Default for Assets<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Buttons<'a> {
    pub fn new() -> Self {
        Self {
            label: None,
            url: None,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn url(mut self, url: &'a str) -> Self {
        self.url = Some(url);
        self
    }
}

impl<'a> Default for Buttons<'a> {
    fn default() -> Self {
        Self::new()
    }
}