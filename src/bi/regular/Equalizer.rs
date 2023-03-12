#[cfg(feature = "BiRegularEqualizer")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularEqualizer")]
/// *This icon requires the feature* `BiRegularEqualizer` *to be enabled*.
#[component]
pub fn Equalizer(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M11 9h2v6h-2zm4-3h2v12h-2zM7 4h2v16H7zm12 7h2v2h-2zM3 10h2v4H3z" /></svg>
   }
}