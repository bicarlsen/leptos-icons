#[cfg(feature = "VsLayoutPanelJustify")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsLayoutPanelJustify")]
/// *This icon requires the feature* `VsLayoutPanelJustify` *to be enabled*.
#[component]
pub fn LayoutPanelJustify(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M2 1L1 2V14L2 15H14L15 14V2L14 1H2ZM2 10V2H4V10H2ZM5 10V2H11V10H5ZM12 10V2H14V10H12Z" /></svg>
   }
}