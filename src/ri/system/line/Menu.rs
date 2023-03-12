#[cfg(feature = "RiSystemLineMenu")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemLineMenu")]
/// *This icon requires the feature* `RiSystemLineMenu` *to be enabled*.
#[component]
pub fn Menu(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M3 4h18v2H3V4zm0 7h18v2H3v-2zm0 7h18v2H3v-2z" /></g></svg>
   }
}