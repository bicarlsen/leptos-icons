#[cfg(feature = "TiLocationArrow")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiLocationArrow")]
/// *This icon requires the feature* `TiLocationArrow` *to be enabled*.
#[component]
pub fn LocationArrow(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M10.368 19.102c.349 1.049 1.011 1.086 1.478.086l5.309-11.375c.467-1.002.034-1.434-.967-.967l-11.376 5.308c-1.001.467-.963 1.129.085 1.479l4.103 1.367 1.368 4.102z" /></svg>
   }
}