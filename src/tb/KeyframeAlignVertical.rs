#[cfg(feature = "TbKeyframeAlignVertical")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbKeyframeAlignVertical")]
/// *This icon requires the feature* `TbKeyframeAlignVertical` *to be enabled*.
#[component]
pub fn KeyframeAlignVertical(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-keyframe-align-vertical" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 2v2" /><path d="M12.816 16.58c-.207 .267 -.504 .42 -.816 .42c-.312 0 -.61 -.153 -.816 -.42l-2.908 -3.748a1.39 1.39 0 0 1 0 -1.664l2.908 -3.748c.207 -.267 .504 -.42 .816 -.42c.312 0 .61 .153 .816 .42l2.908 3.748a1.39 1.39 0 0 1 0 1.664l-2.908 3.748z" /><path d="M12 20v2" /></svg>
   }
}