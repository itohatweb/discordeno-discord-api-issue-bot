use twilight_model::application::component::{select_menu::SelectMenuOption, SelectMenu};

pub const MAX_SELECT_MENU_OPTION_LEN: usize = 25;

#[derive(Clone, Debug)]
#[must_use = "builders have no effect if unused"]
pub struct SelectMenuBuilder(SelectMenu);

impl SelectMenuBuilder {
    /// Create a new builder to construct a [`SelectMenu`].
    pub const fn new(custom_id: String) -> Self {
        Self(SelectMenu {
            custom_id,
            disabled: false,
            max_values: Some(1),
            min_values: Some(1),
            options: vec![],
            placeholder: None,
        })
    }

    /// Consume the builder, returning a [`SelectMenu`].
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn build(self) -> SelectMenu {
        self.0
    }

    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn min_values(mut self, min_values: Option<u8>) -> Self {
        self.0.min_values = match min_values {
            Some(val) if val > 25 => Some(25),
            Some(_) => min_values,
            None => None,
        };

        self
    }

    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn max_values(mut self, max_values: Option<u8>) -> Self {
        self.0.max_values = match max_values {
            Some(val) if val > 25 => Some(25),
            Some(val) if val == 0 => None,
            Some(_) => max_values,
            None => None,
        };

        self
    }

    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.0.disabled = disabled;

        self
    }

    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn placeholder(mut self, placeholder: Option<String>) -> Self {
        self.0.placeholder = placeholder;

        self
    }

    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn add_option(mut self, option: SelectMenuOption) -> Self {
        if self.is_full() {
            return self;
        }

        self.0.options.push(option);

        self
    }

    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn add_options(mut self, options: Vec<SelectMenuOption>) -> Self {
        for option in options.into_iter() {
            if self.is_full() {
                return self;
            }

            self.0.options.push(option)
        }

        self
    }

    fn is_full(&self) -> bool {
        self.0.options.len() == MAX_SELECT_MENU_OPTION_LEN
    }
}
