#[cfg(feature = "FaBrandsUikit")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaBrandsUikit")]
/// *This icon requires the feature* `FaBrandsUikit` *to be enabled*.
#[component]
pub fn Uikit(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M443.9 128v256L218 512 0 384V169.7l87.6 45.1v117l133.5 75.5 135.8-75.5v-151l-101.1-57.6 87.6-53.1L443.9 128zM308.6 49.1L223.8 0l-88.6 54.8 86 47.3 87.4-53z" /></svg>
   }
}