#[cfg(feature = "IoLibrarySharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoLibrarySharp")]
/// *This icon requires the feature* `IoLibrarySharp` *to be enabled*.
#[component]
pub fn LibrarySharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M84,480H28a12,12,0,0,1-12-12V92A12,12,0,0,1,28,80H84A12,12,0,0,1,96,92V468A12,12,0,0,1,84,480Z" /><path d="M240,208V156a12,12,0,0,0-12-12H124a12,12,0,0,0-12,12v52Z" /><path d="M112,416v52a12,12,0,0,0,12,12H228a12,12,0,0,0,12-12V416Z" /><rect x="112" y="240" width="128" height="144" /><path d="M340,480H268a12,12,0,0,1-12-12V44a12,12,0,0,1,12-12h72a12,12,0,0,1,12,12V468A12,12,0,0,1,340,480Z" /><path d="M369,100.7l30,367.83a12,12,0,0,0,13.45,10.92l72.16-9a12,12,0,0,0,10.47-12.9L465,91.21a12,12,0,0,0-13.2-10.94l-72.13,7.51A12,12,0,0,0,369,100.7Z" /></svg>
   }
}