#[cfg(feature = "BiRegularMapAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMapAlt")]
/// *This icon requires the feature* `BiRegularMapAlt` *to be enabled*.
#[component]
pub fn MapAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m21.447 6.105-6-3a1 1 0 0 0-.895 0L9 5.882 3.447 3.105A1 1 0 0 0 2 4v13c0 .379.214.725.553.895l6 3a1 1 0 0 0 .895 0L15 18.118l5.553 2.776a.992.992 0 0 0 .972-.043c.295-.183.475-.504.475-.851V7c0-.379-.214-.725-.553-.895zM10 7.618l4-2v10.764l-4 2V7.618zm-6-2 4 2v10.764l-4-2V5.618zm16 12.764-4-2V5.618l4 2v10.764z" /></svg>
   }
}