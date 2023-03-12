#[cfg(feature = "FaBrandsPatreon")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaBrandsPatreon")]
/// *This icon requires the feature* `FaBrandsPatreon` *to be enabled*.
#[component]
pub fn Patreon(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M512 194.8c0 101.3-82.4 183.8-183.8 183.8-101.7 0-184.4-82.4-184.4-183.8 0-101.6 82.7-184.3 184.4-184.3C429.6 10.5 512 93.2 512 194.8zM0 501.5h90v-491H0v491z" /></svg>
   }
}