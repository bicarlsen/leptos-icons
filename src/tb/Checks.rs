#[cfg(feature = "TbChecks")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbChecks")]
/// *This icon requires the feature* `TbChecks` *to be enabled*.
#[component]
pub fn Checks(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-checks" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M7 12l5 5l10 -10" /><path d="M2 12l5 5m5 -5l5 -5" /></svg>
   }
}