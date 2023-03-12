#[cfg(feature = "FaSolidHelicopterSymbol")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidHelicopterSymbol")]
/// *This icon requires the feature* `FaSolidHelicopterSymbol` *to be enabled*.
#[component]
pub fn HelicopterSymbol(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M443.3 224H508C493.6 108.2 401.8 16.4 286 2V66.7C366.4 80.1 429.9 143.6 443.3 224zM508 288H443.3C429.9 368.4 366.4 431.9 286 445.4V510c115.8-14.4 207.6-106.2 222-222zM0 288C14.4 403.8 106.2 495.6 222 510V445.4C141.6 431.9 78.1 368.4 64.7 288H0zm0-64H64.7C78.1 143.6 141.6 80.1 222 66.7V2C106.2 16.4 14.4 108.2 0 224zm206-64c0-17.7-14.3-32-32-32s-32 14.3-32 32V352c0 17.7 14.3 32 32 32s32-14.3 32-32V288h96v64c0 17.7 14.3 32 32 32s32-14.3 32-32V160c0-17.7-14.3-32-32-32s-32 14.3-32 32v64H206V160z" /></svg>
   }
}