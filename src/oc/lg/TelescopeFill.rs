#[cfg(feature = "OcLgTelescopeFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgTelescopeFill")]
/// *This icon requires the feature* `OcLgTelescopeFill` *to be enabled*.
#[component]
pub fn TelescopeFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M17.155 22.87a.75.75 0 0 0 .226-1.036l-4-6.239a.75.75 0 0 0-.941-.277l-2.75 1.25a.75.75 0 0 0-.318.273l-3.25 4.989a.75.75 0 0 0 1.256.819l3.131-4.806.51-.232v5.64a.75.75 0 1 0 1.5 0v-6.22l3.6 5.613a.75.75 0 0 0 1.036.226ZM.408 15.13a2 2 0 0 1 .59-2.642L17.038 1.33a1.999 1.999 0 0 1 2.85.602l2.828 4.644a2 2 0 0 1-.851 2.847l-17.762 8.43a2 2 0 0 1-2.59-.807Zm13.105-9.521 2.857 4.76 1.361-.646-2.984-4.973Zm-7.842 5.455-1.235.86 1.862 3.225 1.36-.645Z" /></svg>
   }
}