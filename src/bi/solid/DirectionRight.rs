#[cfg(feature = "BiSolidDirectionRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidDirectionRight")]
/// *This icon requires the feature* `BiSolidDirectionRight` *to be enabled*.
#[component]
pub fn DirectionRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m2.295 12.707 8.978 9c.389.39 1.025.391 1.414.002l9.021-9a1 1 0 0 0 0-1.416l-9.021-9a.999.999 0 0 0-1.414.002l-8.978 9a.998.998 0 0 0 0 1.412zm6.707-2.706h5v-2l3 3-3 3v-2h-3v4h-2v-6z" /></svg>
   }
}