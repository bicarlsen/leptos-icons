#[cfg(feature = "BiRegularPhoneIncoming")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularPhoneIncoming")]
/// *This icon requires the feature* `BiRegularPhoneIncoming` *to be enabled*.
#[component]
pub fn PhoneIncoming(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M16.712 13.288a.999.999 0 0 0-1.414 0l-1.597 1.596c-.824-.245-2.166-.771-2.99-1.596-.874-.874-1.374-2.253-1.594-2.992l1.594-1.594a.999.999 0 0 0 0-1.414l-4-4a1.03 1.03 0 0 0-1.414 0l-2.709 2.71c-.382.38-.597.904-.588 1.437.022 1.423.396 6.367 4.297 10.268C10.195 21.6 15.142 21.977 16.566 22h.028c.528 0 1.027-.208 1.405-.586l2.712-2.712a.999.999 0 0 0 0-1.414l-3.999-4zM16.585 20c-1.248-.021-5.518-.356-8.874-3.712C4.343 12.92 4.019 8.636 4 7.414l2.004-2.005L8.59 7.995 7.297 9.288c-.238.238-.34.582-.271.912.024.115.611 2.842 2.271 4.502s4.387 2.247 4.502 2.271a.994.994 0 0 0 .912-.271l1.293-1.293 2.586 2.586L16.585 20z" /><path d="M15.795 6.791 13.005 4v6.995H20l-2.791-2.79 4.503-4.503-1.414-1.414z" /></svg>
   }
}