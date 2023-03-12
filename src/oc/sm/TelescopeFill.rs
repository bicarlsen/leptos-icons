#[cfg(feature = "OcSmTelescopeFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmTelescopeFill")]
/// *This icon requires the feature* `OcSmTelescopeFill` *to be enabled*.
#[component]
pub fn TelescopeFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M11.905.42a1.5 1.5 0 0 1 2.144.49l1.692 2.93a1.5 1.5 0 0 1-.649 2.102L2.895 11.815a1.5 1.5 0 0 1-1.95-.602l-.68-1.176a1.5 1.5 0 0 1 .455-1.99L11.905.422Zm-3.374 9.79a.75.75 0 0 1 .944.253l2.644 3.864a.751.751 0 0 1-1.238.847L9 12.424v2.826a.75.75 0 0 1-1.5 0v-2.826l-1.881 2.75a.75.75 0 1 1-1.238-.848l2.048-2.992a.752.752 0 0 1 .293-.252l1.81-.871Zm2.476-3.965v-.001l1.356-.653-1.52-2.631-1.243.848ZM3.279 8.119l.835 1.445 1.355-.653-.947-1.64Z" /></svg>
   }
}