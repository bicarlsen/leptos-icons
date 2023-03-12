#[cfg(feature = "BiSolidSquareRounded")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidSquareRounded")]
/// *This icon requires the feature* `BiSolidSquareRounded` *to be enabled*.
#[component]
pub fn SquareRounded(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M17 2H7C4.243 2 2 4.243 2 7v10c0 2.757 2.243 5 5 5h10c2.757 0 5-2.243 5-5V7c0-2.757-2.243-5-5-5z" /></svg>
   }
}