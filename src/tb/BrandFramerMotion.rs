#[cfg(feature = "TbBrandFramerMotion")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBrandFramerMotion")]
/// *This icon requires the feature* `TbBrandFramerMotion` *to be enabled*.
#[component]
pub fn BrandFramerMotion(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-brand-framer-motion" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12l-8 -8v16l16 -16v16l-4 -4" /><path d="M20 12l-8 8l-4 -4" /></svg>
   }
}