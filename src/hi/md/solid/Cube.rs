#[cfg(feature = "HiMdSolidCube")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiMdSolidCube")]
/// *This icon requires the feature* `HiMdSolidCube` *to be enabled*.
#[component]
pub fn Cube(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M10.3623 1.09332C10.1368 0.968894 9.86321 0.968894 9.63769 1.09332L2.52344 5.01842L10 9.14342L17.4766 5.01842L10.3623 1.09332ZM18 6.44278L10.75 10.4428V18.6928L17.6123 14.9067C17.8515 14.7747 18 14.5232 18 14.25V6.44278ZM9.25 18.6928V10.4428L2 6.44278V14.25C2 14.5232 2.14852 14.7747 2.38769 14.9067L9.25 18.6928Z" fill="#0F172A" /></svg>
   }
}