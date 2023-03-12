#[cfg(feature = "HiMdSolidCircleStack")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiMdSolidCircleStack")]
/// *This icon requires the feature* `HiMdSolidCircleStack` *to be enabled*.
#[component]
pub fn CircleStack(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M10 1C13.866 1 17 2.79086 17 5C17 7.20914 13.866 9 10 9C6.13401 9 3 7.20914 3 5C3 2.79086 6.13401 1 10 1ZM15.694 9.13079C16.1576 8.86588 16.6044 8.54736 17 8.17775V10C17 12.2091 13.866 14 10 14C6.13401 14 3 12.2091 3 10V8.17775C3.3956 8.54736 3.84244 8.86588 4.30604 9.13079C5.83803 10.0062 7.85433 10.5 10 10.5C12.1457 10.5 14.162 10.0062 15.694 9.13079ZM3 13.1777V15C3 17.2091 6.13401 19 10 19C13.866 19 17 17.2091 17 15V13.1777C16.6044 13.5474 16.1576 13.8659 15.694 14.1308C14.162 15.0062 12.1457 15.5 10 15.5C7.85433 15.5 5.83803 15.0062 4.30604 14.1308C3.84244 13.8659 3.3956 13.5474 3 13.1777Z" fill="#0F172A" /></svg>
   }
}