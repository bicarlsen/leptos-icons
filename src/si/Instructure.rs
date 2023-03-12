#[cfg(feature = "SiInstructure")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiInstructure")]
/// *This icon requires the feature* `SiInstructure` *to be enabled*.
#[component]
pub fn Instructure(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="m11.996 0-5.11 2.878L12 5.76l5.115-2.878ZM6.032 3.36.918 6.237 6.036 9.12l5.115-2.879Zm11.929 0-5.112 2.878 5.115 2.882 5.118-2.879zM12 11.52.918 17.76 12 24l11.082-6.241Z" /></svg>
   }
}