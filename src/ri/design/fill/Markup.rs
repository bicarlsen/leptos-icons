#[cfg(feature = "RiDesignFillMarkup")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDesignFillMarkup")]
/// *This icon requires the feature* `RiDesignFillMarkup` *to be enabled*.
#[component]
pub fn Markup(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 22C6.477 22 2 17.523 2 12S6.477 2 12 2s10 4.477 10 10-4.477 10-10 10zm5.051-3.796l-.862-3.447a1 1 0 0 0-.97-.757H8.781a1 1 0 0 0-.97.757l-.862 3.447A7.967 7.967 0 0 0 12 20a7.967 7.967 0 0 0 5.051-1.796zM10 12h4v-1.5l-1.038-3.635a1 1 0 0 0-1.924 0L10 10.5V12z" /></g></svg>
   }
}