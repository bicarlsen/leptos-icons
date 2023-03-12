#[cfg(feature = "FaSolidChildCombatant")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidChildCombatant")]
/// *This icon requires the feature* `FaSolidChildCombatant` *to be enabled*.
#[component]
pub fn ChildCombatant(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M144 128A64 64 0 1 0 144 0a64 64 0 1 0 0 128zm-8 352V352h16V480c0 17.7 14.3 32 32 32s32-14.3 32-32V300.5L228.9 321c9.4 15 29.2 19.4 44.1 10s19.4-29.2 10-44.1l-51.7-82.1c-17.6-27.9-48.3-44.9-81.2-44.9H137.8c-33 0-63.7 16.9-81.2 44.9L4.9 287c-9.4 15-4.9 34.7 10 44.1s34.7 4.9 44.1-10L72 300.5V480c0 17.7 14.3 32 32 32s32-14.3 32-32zM416 0H400 384c-8.8 0-16 7.2-16 16s7.2 16 16 16V132.3c-9.6 5.5-16 15.9-16 27.7v32c-17.7 0-32 14.3-32 32V368c0 17.7 14.3 32 32 32h16v96c0 8.8 7.2 16 16 16h59.5c10.4 0 18-9.8 15.5-19.9L452 400h44c8.8 0 16-7.2 16-16V368c0-8.8-7.2-16-16-16H448V325.3l53.1-17.7c6.5-2.2 10.9-8.3 10.9-15.2V208c0-8.8-7.2-16-16-16H480c-8.8 0-16 7.2-16 16v56l-16 5.3V160c0-11.8-6.4-22.2-16-27.7V16c0-8.8-7.2-16-16-16z" /></svg>
   }
}