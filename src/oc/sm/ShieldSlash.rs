#[cfg(feature = "OcSmShieldSlash")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmShieldSlash")]
/// *This icon requires the feature* `OcSmShieldSlash` *to be enabled*.
#[component]
pub fn ShieldSlash(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M8.533.133a1.75 1.75 0 0 0-1.066 0l-2.091.67a.75.75 0 0 0 .457 1.428l2.09-.67a.25.25 0 0 1 .153 0l5.25 1.68a.25.25 0 0 1 .174.239V7c0 .233-.008.464-.025.694a.75.75 0 1 0 1.495.112c.02-.27.03-.538.03-.806V3.48a1.75 1.75 0 0 0-1.217-1.667L8.533.133ZM1 2.857l-.69-.5a.75.75 0 1 1 .88-1.214l14.5 10.5a.75.75 0 1 1-.88 1.214l-1.282-.928c-.995 1.397-2.553 2.624-4.864 3.608-.425.181-.905.18-1.329 0-2.447-1.042-4.049-2.356-5.032-3.855C1.32 10.182 1 8.566 1 7Zm1.5 1.086V7c0 1.358.275 2.666 1.057 3.86.784 1.194 2.121 2.34 4.366 3.297.05.02.106.02.153 0 2.127-.905 3.439-1.982 4.237-3.108Z" /></svg>
   }
}