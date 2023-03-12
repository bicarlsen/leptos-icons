#[cfg(feature = "BiSolidMessageRoundedMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMessageRoundedMinus")]
/// *This icon requires the feature* `BiSolidMessageRoundedMinus` *to be enabled*.
#[component]
pub fn MessageRoundedMinus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 5.589 2 10c0 2.907 1.897 5.515 5 6.934V22l5.34-4.005C17.697 17.853 22 14.32 22 10c0-4.411-4.486-8-10-8zm4 9H8V9h8v2z" /></svg>
   }
}