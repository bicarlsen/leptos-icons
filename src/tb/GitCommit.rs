#[cfg(feature = "TbGitCommit")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbGitCommit")]
/// *This icon requires the feature* `TbGitCommit` *to be enabled*.
#[component]
pub fn GitCommit(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-git-commit" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /><path d="M12 3l0 6" /><path d="M12 15l0 6" /></svg>
   }
}