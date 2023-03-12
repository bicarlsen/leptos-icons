#[cfg(feature = "BiSolidFoodMenu")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidFoodMenu")]
/// *This icon requires the feature* `BiSolidFoodMenu` *to be enabled*.
#[component]
pub fn FoodMenu(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 2h2v20H3zm16 0H6v20h13c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zm-1 10H9v-2h9v2zm0-4H9V6h9v2z" /></svg>
   }
}