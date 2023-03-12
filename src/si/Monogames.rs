#[cfg(feature = "SiMonogames")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiMonogames")]
/// *This icon requires the feature* `SiMonogames` *to be enabled*.
#[component]
pub fn Monogames(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M24 10.356V7.083a7.083 7.083 0 0 0-12-5.098A7.083 7.083 0 0 0 0 7.083v9.834A7.083 7.083 0 0 0 7.083 24h9.834a7.083 7.083 0 0 0 7.07-7.51H24v-4.332H9.834v4.332h9.834v.427a2.751 2.751 0 0 1-2.751 2.751H7.083a2.751 2.751 0 0 1-2.751-2.751V7.083a2.751 2.751 0 1 1 5.502 0v3.273h4.332V7.083a2.751 2.751 0 1 1 5.502 0v3.273z" /></svg>
   }
}