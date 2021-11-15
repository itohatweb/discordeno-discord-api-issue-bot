use twilight_model::application::component::{ActionRow, Button, Component, SelectMenu};

#[derive(Clone, Debug)]
#[must_use = "builders have no effect if unused"]
pub struct ComponentBuilder(Vec<Component>);

impl ComponentBuilder {
    /// Create a new builder to construct a Vec<[`Component`]>.
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Consume the builder, returning a Vec<[`Component`]>.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn build(self) -> Vec<Component> {
        self.0
    }

    pub fn button(mut self, button: Button) -> Self {
        // Get the last ActionRow
        let component = self
            .0
            .iter_mut()
            .rev()
            .find(|component| matches!(component, Component::ActionRow(_)));

        match component {
            // Check if the ActionRow still has place and then add the button
            Some(Component::ActionRow(action_row))
                if action_row.components.len() < 5
                // Currently only buttons are allowed to be in the same ActionRow
                    && action_row.components.iter().all(|c| match c {
                        Component::Button(_) => true,
                        _ => false,
                    }) =>
            {
                action_row.components.push(Component::Button(button));

                self
            }
            // No ActionRow found or its full so create a new one
            _ => self.action_row(vec![Component::Button(button)]),
        }
    }

    pub fn select_menu(self, select_menu: SelectMenu) -> Self {
        self.action_row(vec![Component::SelectMenu(select_menu)])
    }

    pub fn action_row(mut self, components: Vec<Component>) -> Self {
        if self.is_full() {
            return self;
        }

        self.0.push(Component::ActionRow(ActionRow { components }));

        self
    }

    fn is_full(&self) -> bool {
        self.0.len() == 5
    }
}
