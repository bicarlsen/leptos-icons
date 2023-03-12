#[cfg(feature = "SiXmpp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiXmpp")]
/// *This icon requires the feature* `SiXmpp` *to be enabled*.
#[component]
pub fn Xmpp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M24 3.186l-3.217 1.248-.976.325-3.122.925c.014.18.014.361.014.555 0 3.422-1.744 7.59-4.63 10.573-2.805-2.972-4.49-7.066-4.49-10.434 0-.194 0-.375.014-.555l-3.11-.921v-.009l-.861-.306L0 3.172c.146 5.747 4.867 11.701 10.542 15.02-1.3 1.014-2.766 1.788-4.365 2.192v.319c.434-.054.852-.14 1.271-.225.15-.027.3-.048.451-.08l.047-.012c1.36-.29 2.722-.776 4.052-1.408.397.195.797.38 1.2.548.109.05.22.095.33.142.201.086.407.159.612.236 1.25.474 2.568.809 3.96.924v-.305c-1.68-.425-3.211-1.264-4.56-2.355C19.194 14.904 23.853 8.975 24 3.186z" /></svg>
   }
}