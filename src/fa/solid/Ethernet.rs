#[cfg(feature = "FaSolidEthernet")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidEthernet")]
/// *This icon requires the feature* `FaSolidEthernet` *to be enabled*.
#[component]
pub fn Ethernet(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M0 224V416c0 17.7 14.3 32 32 32H96V336c0-8.8 7.2-16 16-16s16 7.2 16 16V448h64V336c0-8.8 7.2-16 16-16s16 7.2 16 16V448h64V336c0-8.8 7.2-16 16-16s16 7.2 16 16V448h64V336c0-8.8 7.2-16 16-16s16 7.2 16 16V448h64c17.7 0 32-14.3 32-32V224c0-17.7-14.3-32-32-32H448V160c0-17.7-14.3-32-32-32H384V96c0-17.7-14.3-32-32-32H160c-17.7 0-32 14.3-32 32v32H96c-17.7 0-32 14.3-32 32v32H32c-17.7 0-32 14.3-32 32z" /></svg>
   }
}