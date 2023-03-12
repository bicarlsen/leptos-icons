#[cfg(feature = "TbHexagon6Filled")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbHexagon6Filled")]
/// *This icon requires the feature* `TbHexagon6Filled` *to be enabled*.
#[component]
pub fn Hexagon6Filled(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-hexagon-6-filled" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M10.543 2.426c.862 -.48 1.9 -.512 2.785 -.096l.187 .096l6.026 3.588l.095 .063l.084 .07l.113 .083a3 3 0 0 1 1.143 1.992l.019 .198l.005 .2v6.536c0 1.022 -.52 1.968 -1.326 2.492l-.165 .099l-6.053 3.864c-.845 .47 -1.86 .501 -2.772 .069l-.193 -.1l-5.947 -3.802a3 3 0 0 1 -1.537 -2.418l-.007 -.203v-6.537c0 -1.022 .52 -1.968 1.348 -2.505l6.195 -3.689zm2.457 4.574h-2l-.15 .005a2 2 0 0 0 -1.844 1.838l-.006 .157v6l.005 .15a2 2 0 0 0 1.838 1.844l.157 .006h2l.15 -.005a2 2 0 0 0 1.844 -1.838l.006 -.157v-2l-.005 -.15a2 2 0 0 0 -1.838 -1.844l-.157 -.006h-2v-2h2l.007 .117a1 1 0 0 0 1.993 -.117a2 2 0 0 0 -1.85 -1.995l-.15 -.005zm0 6v2h-2v-2h2z" stroke-width="0" fill="currentColor" /></svg>
   }
}