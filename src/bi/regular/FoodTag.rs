#[cfg(feature = "BiRegularFoodTag")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularFoodTag")]
/// *This icon requires the feature* `BiRegularFoodTag` *to be enabled*.
#[component]
pub fn FoodTag(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 3H4a1 1 0 0 0-1 1v16a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1V4a1 1 0 0 0-1-1zm-1 16H5V5h14v14z" /><circle cx="12" cy="12" r="5" /></svg>
   }
}