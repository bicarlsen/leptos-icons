#[cfg(feature = "BiRegularCircleQuarter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCircleQuarter")]
/// *This icon requires the feature* `BiRegularCircleQuarter` *to be enabled*.
#[component]
pub fn CircleQuarter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2h-1v11h11v-1A10 10 0 0 0 12 2zm1 9V4.06A8 8 0 0 1 19.94 11z" /></svg>
   }
}