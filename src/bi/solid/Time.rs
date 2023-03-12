#[cfg(feature = "BiSolidTime")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidTime")]
/// *This icon requires the feature* `BiSolidTime` *to be enabled*.
#[component]
pub fn Time(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12.25 2c-5.514 0-10 4.486-10 10s4.486 10 10 10 10-4.486 10-10-4.486-10-10-10zM18 13h-6.75V6h2v5H18v2z" /></svg>
   }
}