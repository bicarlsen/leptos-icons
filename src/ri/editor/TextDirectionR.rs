#[cfg(feature = "RiEditorTextDirectionR")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorTextDirectionR")]
/// *This icon requires the feature* `RiEditorTextDirectionR` *to be enabled*.
#[component]
pub fn TextDirectionR(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M11 5v10H9v-4a4 4 0 1 1 0-8h8v2h-2v10h-2V5h-2zM9 5a2 2 0 1 0 0 4V5zM7 17h12v2H7v2.5L3 18l4-3.5V17z" /></g></svg>
   }
}