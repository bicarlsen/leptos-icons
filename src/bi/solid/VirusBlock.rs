#[cfg(feature = "BiSolidVirusBlock")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidVirusBlock")]
/// *This icon requires the feature* `BiSolidVirusBlock` *to be enabled*.
#[component]
pub fn VirusBlock(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18.952 17.538c-.749-.749-.908-1.869-.5-2.846l.021-.049c.399-.974 1.309-1.643 2.362-1.643h.08c.638 0 1.085-.447 1.085-1s-.447-1-1-1h-.17c-1.053 0-1.958-.669-2.357-1.644l-.021-.049c-.408-.977-.249-2.097.5-2.846l.119-.119a.999.999 0 1 0-1.414-1.414l-.119.119c-.749.749-1.869.908-2.846.5l-.049-.021C13.669 5.128 13 4.218 13 3.165v-.081C13 2.447 12.553 2 12 2s-1 .447-1 1v.036c0 1.096-.66 2.084-1.673 2.503l-.006.003a2.71 2.71 0 0 1-2.953-.588l-.025-.025-2.636-2.636-1.414 1.414 18 18 1.414-1.414-2.636-2.636-.119-.119zM12 10a2 2 0 1 1 2 2c-.257 0-.501-.053-.728-.142l-1.131-1.131A1.998 1.998 0 0 1 12 10zm-4 3a1 1 0 0 1-1-1 .99.99 0 0 1 .244-.635L5.431 9.552A2.634 2.634 0 0 1 3.085 11h-.001C2.447 11 2 11.447 2 12s.447 1 1 1h.068a2.66 2.66 0 0 1 2.459 1.644l.021.049a2.69 2.69 0 0 1-.583 2.929l-.036.036a.999.999 0 1 0 1.414 1.414l.036-.036a2.689 2.689 0 0 1 2.929-.583l.143.06A2.505 2.505 0 0 1 11 20.83v.085c0 .638.447 1.085 1 1.085s1-.448 1-1v-.17c0-.976.568-1.853 1.443-2.266l-5.809-5.809A.98.98 0 0 1 8 13z" /></svg>
   }
}