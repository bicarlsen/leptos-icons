#[cfg(feature = "AiOutlinedSmallDash")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiOutlinedSmallDash")]
/// *This icon requires the feature* `AiOutlinedSmallDash` *to be enabled*.
#[component]
pub fn SmallDash(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 1024 1024"><path d="M112 476h72v72h-72zm182 0h72v72h-72zm364 0h72v72h-72zm182 0h72v72h-72zm-364 0h72v72h-72z" /></svg>
   }
}