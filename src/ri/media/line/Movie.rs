#[cfg(feature = "RiMediaLineMovie")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMediaLineMovie")]
/// *This icon requires the feature* `RiMediaLineMovie` *to be enabled*.
#[component]
pub fn Movie(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M2 3.993A1 1 0 0 1 2.992 3h18.016c.548 0 .992.445.992.993v16.014a1 1 0 0 1-.992.993H2.992A.993.993 0 0 1 2 20.007V3.993zM4 5v14h16V5H4zm6.622 3.415l4.879 3.252a.4.4 0 0 1 0 .666l-4.88 3.252a.4.4 0 0 1-.621-.332V8.747a.4.4 0 0 1 .622-.332z" /></g></svg>
   }
}