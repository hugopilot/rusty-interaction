use serde::{Deserialize, Serialize};

use ::chrono::{DateTime, Utc};
use serde_with::*;

#[cfg(feature = "builder")]
use crate::Builder;
#[cfg(feature = "builder")]
use log::warn;
// ======== Structures =========
#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// An embed in Discord is a way to display rich content in messages
pub struct Embed {
    /// Title of the embed
    pub title: Option<String>,
    /// Description of the embed
    pub description: Option<String>,
    // Type field is not implemented since it's considered deprecated
    /// url of embed
    pub url: Option<String>,
    #[serde(default)]
    /// Timestamp of embed content
    pub timestamp: Option<DateTime<Utc>>,
    /// Color code of embed
    pub color: Option<u32>,
    /// Footer information
    pub footer: Option<EmbedFooter>,
    /// Image information
    pub image: Option<EmbedImage>,
    /// Thumbnail information
    pub thumbnail: Option<EmbedThumbnail>,
    /// Video information
    pub video: Option<EmbedVideo>,
    /// Provider information
    pub provider: Option<EmbedProvider>,
    /// Author information
    pub author: Option<EmbedAuthor>,
    /// Fields of the embed
    pub fields: Option<Vec<EmbedField>>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
/// Representing a Thumbnail for an [`Embed`]
pub struct EmbedThumbnail {
    /// Url of the thumbnail
    pub url: Option<String>,
    /// Proxied url of the thumbnail
    pub proxy_url: Option<String>,
    /// Height of the image
    pub height: Option<u32>,
    /// Width of the image
    pub width: Option<u32>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Representing video information in an [`Embed`]
pub struct EmbedVideo {
    url: String,
    proxy_url: String,
    height: i32,
    witdh: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
/// Representing image information in an [`Embed`]
pub struct EmbedImage {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    pub height: i32,
    pub width: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Representing provider information in an [`Embed`]
pub struct EmbedProvider {
    name: String,
    url: String,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
/// Representing the author of an [`Embed`]
pub struct EmbedAuthor {
    /// Name of author
    pub name: Option<String>,
    /// Url of author
    pub url: Option<String>,
    /// Url of author icon (only supports http(s) and attachments)
    pub icon_url: Option<String>,
    /// A proxied url of author icon
    pub proxy_icon_url: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Representing the footer of an [`Embed`]
pub struct EmbedFooter {
    /// Footer text
    text: String,
    /// Url of footer icon (only supports http(s) and attachments)
    icon_url: Option<String>,
    /// A proxied url of footer icon
    proxy_icon_url: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Representing a field of an [`Embed`]
pub struct EmbedField {
    /// Name of the field
    name: String,
    /// Value of the field
    value: String,
    /// Whether or not this field should display inline
    #[serde(skip_serializing_if = "Option::is_none")]
    inline: Option<bool>,
}
#[cfg(feature = "builder")]
#[derive(Clone, Default, Debug, PartialEq)]
/// Builder to construct an [`Embed`]
pub struct EmbedBuilder {
    obj: Embed,
}
#[derive(Clone, Copy, Debug, PartialEq)]
/// Representing RGB colors.
///
/// Each color is an 8bit unsigned integer.
pub struct Color {
    /// Red value
    pub red: u8,
    /// Green value
    pub green: u8,
    /// Blue value
    pub blue: u8,
}

// ========== IMPLS ===========
impl Default for Color {
    fn default() -> Self {
        Color {
            // ;)
            red: 222,
            green: 165,
            blue: 132,
        }
    }
}

impl From<u32> for Color {
    fn from(a: u32) -> Color {
        Color {
            red: ((a >> 16) & 0xff) as u8,
            green: ((a >> 8) & 0xff) as u8,
            blue: (a & 0xff) as u8,
        }
    }
}

impl From<Color> for u32 {
    fn from(c: Color) -> u32 {
        ((c.red as u32) << 16) | ((c.green as u32) << 8) | c.blue as u32
    }
}

impl Default for Embed {
    fn default() -> Self {
        Self {
            title: None,
            description: None,
            url: None,
            timestamp: None,
            color: Some(Color::default().into()),
            footer: None,
            image: None,
            thumbnail: None,
            video: None,
            provider: None,
            author: None,
            fields: None,
        }
    }
}
#[cfg(feature = "builder")]
impl EmbedBuilder {
    /// Set the title of this embed
    pub fn title(mut self, title: impl ToString) -> Self {
        let t = title.to_string();
        // wish this could be checked at compile time :(
        if t.len() > 256 {
            panic!("Embed title length is more than 256 characters.")
        }
        self.obj.title = Some(t);
        self
    }

    /// Add a description to the embed
    pub fn description(mut self, description: impl ToString) -> Self {
        let d = description.to_string();

        self.obj.description = Some(d);
        self
    }

    /// Set the url of the title of this embed
    pub fn url(mut self, url: &str) -> Self {
        self.obj.url = Some(String::from(url));
        self
    }
    /// Set the color of this embed
    pub fn color(mut self, color: impl Into<u32>) -> Self {
        self.obj.color = Some(color.into());
        self
    }

    /// Set the timestamp of this embed
    pub fn timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.obj.timestamp = Some(timestamp);
        self
    }

    /// Set the embed's footer
    pub fn footer(mut self, a: EmbedFooter) -> Self {
        self.obj.footer = Some(a);
        self
    }

    /// Set the embed author
    pub fn author(mut self, author: EmbedAuthor) -> Self {
        self.obj.author = Some(author);
        self
    }

    pub fn image(mut self, image: EmbedImage) -> Self {
        self.obj.image = Some(image);
        self
    }

    pub fn thumbnail(mut self, thumbnail: EmbedThumbnail) -> Self {
        self.obj.thumbnail = Some(thumbnail);
        self
    }

    /// Add an [`EmbedField`] to this embed.
    pub fn add_field(mut self, field: EmbedField) -> Self {
        match self.obj.fields {
            None => {
                let nf = Some(vec![field]);
                self.obj.fields = nf;
            }
            Some(ref mut f) => {
                if f.len() >= 25 {
                    warn!("Field limit reached. Ignoring");
                } else {
                    f.push(field);
                }
            }
        }
        self
    }

    #[deprecated(since = "0.1.9", note = "Use the `build()` function instead")]
    /// Build the embed. You can't use the function after this anymore
    pub fn finish(self) -> Embed {
        self.obj
    }
}

#[cfg(feature = "builder")]
impl Builder<Embed> for EmbedBuilder {
    type Error = std::convert::Infallible;

    fn build(self) -> Result<Embed, Self::Error> {
        Ok(self.obj)
    }
}

impl Default for EmbedFooter {
    fn default() -> Self {
        Self {
            text: String::from(""),
            icon_url: None,
            proxy_icon_url: None,
        }
    }
}

impl EmbedFooter {
    /// Set the footers text
    pub fn text(mut self, text: impl ToString) -> Self {
        let t = text.to_string();
        if t.len() > 2048 {
            panic!("Footer text exceeded 2048 characters")
        }
        self.text = t;
        self
    }

    /// Sets the url to the footer icon
    pub fn icon_url(mut self, url: impl ToString) -> Self {
        let n = url.to_string();

        self.icon_url = Some(n);
        self
    }

    /// Sets a proxied url to the footer icon
    pub fn proxy_url(mut self, url: impl ToString) -> Self {
        let u = url.to_string();

        self.proxy_icon_url = Some(u);
        self
    }
}

impl Default for EmbedField {
    fn default() -> Self {
        Self {
            value: String::from(""),
            name: String::from(""),
            inline: None,
        }
    }
}

impl EmbedField {
    /// Set the field name
    pub fn name(mut self, name: impl ToString) -> Self {
        let n = name.to_string();
        if n.len() > 256 {
            panic!("Field name is above 256 characters.")
        }
        self.name = n;
        self
    }

    /// Set the text of this field
    pub fn value(mut self, text: impl ToString) -> Self {
        let t = text.to_string();

        if t.len() > 1024 {
            panic!("Field value is above 1024 characters")
        }
        self.value = t;
        self
    }
    /// Set if the field should display inline
    pub fn inline(mut self, inline: bool) -> Self {
        self.inline = Some(inline);
        self
    }
}

impl EmbedAuthor {
    /// Set the author name
    pub fn name(mut self, name: impl ToString) -> Self {
        let n = name.to_string();
        self.name = Some(n);
        self
    }

    /// Sets the URL users can click on.
    pub fn url(mut self, url: impl ToString) -> Self {
        let n = url.to_string();
        self.url = Some(n);
        self
    }

    /// Add an icon to the embed
    pub fn icon_url(mut self, url: impl ToString) -> Self {
        let u = url.to_string();

        self.icon_url = Some(u);
        self
    }

    /// Set the proxy url for the icon
    pub fn proxy_url(mut self, url: impl ToString) -> Self {
        let u = url.to_string();

        self.proxy_icon_url = Some(u);
        self
    }
}

impl EmbedThumbnail {
    /// Sets the URL of the thumbnail
    pub fn url(mut self, url: impl ToString) -> Self {
        let u = url.to_string();
        self.url = Some(u);
        self
    }

    /// Sets a proxied url for the thumbnail
    pub fn proxy_url(mut self, url: impl ToString) -> Self {
        let u = url.to_string();
        self.url = Some(u);
        self
    }

    /// Sets the dimensions of the thumbnail
    pub fn dimensions(mut self, height: impl Into<u32>, width: impl Into<u32>) -> Self {
        let x = width.into();
        let y = height.into();

        self.height = Some(y);
        self.width = Some(x);

        self
    }
}
