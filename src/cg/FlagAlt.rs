#[cfg(feature = "CgFlagAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgFlagAlt")]
/// *This icon requires the feature* `CgFlagAlt` *to be enabled*.
#[component]
pub fn FlagAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M12.4388 7L14.8387 4H7V10H14.8387L12.4388 7ZM19 12H7V22H5V2H19L15 7L19 12Z" fill="currentColor" /></svg>
   }
}