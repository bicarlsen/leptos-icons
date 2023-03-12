#[cfg(feature = "AiOutlinedSafety")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiOutlinedSafety")]
/// *This icon requires the feature* `AiOutlinedSafety` *to be enabled*.
#[component]
pub fn Safety(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024"><path d="M512 64L128 192v384c0 212.1 171.9 384 384 384s384-171.9 384-384V192L512 64zm312 512c0 172.3-139.7 312-312 312S200 748.3 200 576V246l312-110 312 110v330z" /><path d="M378.4 475.1a35.91 35.91 0 0 0-50.9 0 35.91 35.91 0 0 0 0 50.9l129.4 129.4 2.1 2.1a33.98 33.98 0 0 0 48.1 0L730.6 434a33.98 33.98 0 0 0 0-48.1l-2.8-2.8a33.98 33.98 0 0 0-48.1 0L483 579.7 378.4 475.1z" /></svg>
   }
}