#[cfg(feature = "ImSortAlphaDesc")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImSortAlphaDesc")]
/// *This icon requires the feature* `ImSortAlphaDesc` *to be enabled*.
#[component]
pub fn SortAlphaDesc(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M5 12v-12h-2v12h-2.5l3.5 3.5 3.5-3.5h-2.5z" /><path fill="#000000" d="M14.5 7h-4c-0.184 0-0.354-0.101-0.441-0.264s-0.077-0.36 0.025-0.513l3.482-5.223h-3.066c-0.276 0-0.5-0.224-0.5-0.5s0.224-0.5 0.5-0.5h4c0.184 0 0.354 0.102 0.441 0.264s0.077 0.36-0.025 0.513l-3.482 5.223h3.066c0.276 0 0.5 0.224 0.5 0.5s-0.224 0.5-0.5 0.5z" /><path fill="#000000" d="M15.947 15.276l-3-6c-0.085-0.169-0.258-0.276-0.447-0.276s-0.363 0.107-0.447 0.276l-3 6c-0.123 0.247-0.023 0.547 0.224 0.671 0.072 0.036 0.148 0.053 0.223 0.053 0.183 0 0.36-0.101 0.448-0.277l0.862-1.724h3.382l0.862 1.724c0.123 0.247 0.424 0.347 0.671 0.224s0.347-0.424 0.224-0.671zM11.309 13l1.191-2.382 1.191 2.382h-2.382z" /></svg>
   }
}