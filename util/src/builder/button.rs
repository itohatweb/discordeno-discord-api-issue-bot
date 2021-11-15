use twilight_model::{
    application::component::{button::ButtonStyle, Button},
    channel::ReactionType,
};

#[derive(Clone, Debug)]
#[must_use = "builders have no effect if unused"]
pub struct ButtonBuilder(Button);

impl ButtonBuilder {
    /// Create a new builder to construct a [`Button`].
    pub const fn new(style: ButtonStyle, custom_id_or_url: String) -> Self {
        Self(Button {
            style,
            emoji: None,
            label: None,
            custom_id: Some(custom_id_or_url),
            url: None,
            disabled: false,
        })
    }

    /// Consume the builder, returning a [`Button`].
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn build(mut self) -> Button {
        if self.0.style == ButtonStyle::Link {
            self.0.url = self.0.custom_id;
            self.0.custom_id = None;
        }

        self.0
    }

    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn label(mut self, label: String) -> Self {
        self.0.label = Some(label);

        self
    }

    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn emoji(mut self, emoji: ReactionType) -> Self {
        self.0.emoji = Some(emoji);

        self
    }

    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.0.disabled = disabled;

        self
    }
}
