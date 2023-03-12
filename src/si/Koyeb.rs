#[cfg(feature = "SiKoyeb")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiKoyeb")]
/// *This icon requires the feature* `SiKoyeb` *to be enabled*.
#[component]
pub fn Koyeb(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 12.822V7.044L11.985.115 24 7.037v5.77L11.992 5.892Zm11.985 1.114L1.92 19.759 0 18.645v-3.557l11.985-6.93L24 15.089v3.542l-1.92 1.13Zm-3.028 9.949L3.95 21.004l8.036-4.656 8.066 4.656-5.009 2.88-3.05-1.759Z" /></svg>
   }
}