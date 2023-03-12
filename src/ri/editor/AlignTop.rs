#[cfg(feature = "RiEditorAlignTop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorAlignTop")]
/// *This icon requires the feature* `RiEditorAlignTop` *to be enabled*.
#[component]
pub fn AlignTop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M3 3h18v2H3V3zm5 8v10H6V11H3l4-4 4 4H8zm10 0v10h-2V11h-3l4-4 4 4h-3z" /></g></svg>
   }
}