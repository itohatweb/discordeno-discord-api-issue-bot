use twilight_model::{
    application::component::select_menu::SelectMenuOption, channel::ReactionType,
};

pub const MAX_SELECT_MENU_OPTION_LEN: usize = 25;

#[derive(Clone, Debug)]
#[must_use = "builders have no effect if unused"]
pub struct SelectMenuOptionBuilder(SelectMenuOption);

impl SelectMenuOptionBuilder {
    /// Create a new builder to construct a [`SelectMenuOption`].
    pub const fn new(label: String, value: String) -> Self {
        Self(SelectMenuOption {
            default: false,
            description: None,
            emoji: None,
            label,
            value,
        })
    }

    /// Consume the builder, returning a [`SelectMenuOption`].
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn build(self) -> SelectMenuOption {
        self.0
    }

    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn default(mut self, default: bool) -> Self {
        self.0.default = default;

        self
    }

    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn description(mut self, description: Option<String>) -> Self {
        self.0.description = description;

        self
    }

    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn emoji(mut self, emoji: ReactionType) -> Self {
        self.0.emoji = Some(emoji);

        self
    }
}
