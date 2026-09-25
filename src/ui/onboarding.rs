use ratatui::layout::Rect;

pub(crate) const ONBOARDING_TITLE: &str = "  herdr";
pub(crate) const ONBOARDING_PREFIX_LABEL: &str = "ctrl+b";
pub(crate) const ONBOARDING_HELP_LABEL: &str = "?";

pub(crate) fn onboarding_welcome_continue_rect(area: Rect) -> Rect {
    super::widgets::continue_button_rect(area)
}
