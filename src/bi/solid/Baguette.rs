#[cfg(feature = "BiSolidBaguette")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBaguette")]
/// *This icon requires the feature* `BiSolidBaguette` *to be enabled*.
#[component]
pub fn Baguette(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m11.13 4.41 4.23 4.23L14.3 9.7l-4.24-4.24-1.77 1.77 4.24 4.24-1.06 1.06-4.24-4.24-1.77 1.77L9.7 14.3l-1.06 1.06-4.23-4.23C1.86 14 1.55 18 3.79 20.21a5.38 5.38 0 0 0 3.85 1.5 8 8 0 0 0 5.6-2.47l6-6c2.87-2.87 3.31-7.11 1-9.45s-6.24-1.93-9.11.62z" /></svg>
   }
}