#[cfg(feature = "RiBusinessLineTrademark")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiBusinessLineTrademark")]
/// *This icon requires the feature* `RiBusinessLineTrademark` *to be enabled*.
#[component]
pub fn Trademark(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M10 6v2H6v10H4V8H0V6h10zm2 0h2.5l3 5.196L20.5 6H23v12h-2V9.133l-3.5 6.063L14 9.135V18h-2V6z" /></g></svg>
   }
}