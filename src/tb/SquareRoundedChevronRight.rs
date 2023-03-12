#[cfg(feature = "TbSquareRoundedChevronRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbSquareRoundedChevronRight")]
/// *This icon requires the feature* `TbSquareRoundedChevronRight` *to be enabled*.
#[component]
pub fn SquareRoundedChevronRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-square-rounded-chevron-right" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M11 9l3 3l-3 3" /><path d="M12 3c7.2 0 9 1.8 9 9s-1.8 9 -9 9s-9 -1.8 -9 -9s1.8 -9 9 -9z" /></svg>
   }
}