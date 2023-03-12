#[cfg(feature = "RiSystemLineArrowGoBack")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemLineArrowGoBack")]
/// *This icon requires the feature* `RiSystemLineArrowGoBack` *to be enabled*.
#[component]
pub fn ArrowGoBack(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M5.828 7l2.536 2.536L6.95 10.95 2 6l4.95-4.95 1.414 1.414L5.828 5H13a8 8 0 1 1 0 16H4v-2h9a6 6 0 1 0 0-12H5.828z" /></g></svg>
   }
}