#[cfg(feature = "BiLogosKubernetes")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiLogosKubernetes")]
/// *This icon requires the feature* `BiLogosKubernetes` *to be enabled*.
#[component]
pub fn Kubernetes(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m10.013 13.738-2.254.387a4.475 4.475 0 0 0 1.753 2.231l.865-2.095a.27.27 0 0 0 .022-.228c-.022-.18-.204-.295-.386-.295zm.865-2.914c.205 0 .387-.159.387-.364l.114-2.277-.456.091a4.516 4.516 0 0 0-2.118 1.162l1.89 1.343c.069.022.115.045.183.045zm-1.002 1.48a.366.366 0 0 0-.045-.524l-1.685-1.503a4.797 4.797 0 0 0-.661 2.778l2.209-.637c.091-.023.137-.046.182-.114zm1.457.797.616.296.614-.296.16-.661-.433-.546h-.683l-.433.546zm1.344-2.436c.114.159.341.182.5.091l1.867-1.32a4.286 4.286 0 0 0-2.573-1.23l.137 2.3a.215.215 0 0 0 .069.159z" /><path d="m21.944 14.103-1.73-7.446c-.113-.364-.387-.66-.729-.842L12.541 2.49c-.182-.091-.387-.114-.569-.114s-.387 0-.569.045L4.457 5.769a1.22 1.22 0 0 0-.683.842l-1.708 7.492c-.068.387.023.774.25 1.093l4.805 5.943c.273.273.66.456 1.047.479h7.651c.41.045.797-.137 1.048-.479l4.805-5.943c.227-.319.318-.706.272-1.093zm-2.845.501c-.046 0-.068 0-.114-.023-.022-.023-.022-.023-.045-.023-.046 0-.068-.022-.092-.022-.091-.023-.159-.068-.25-.114a.32.32 0 0 1-.137-.045h-.022a3.91 3.91 0 0 0-.729-.205h-.022a.26.26 0 0 0-.182.068s0 .023-.023.023l-.183-.024a5.628 5.628 0 0 1-2.46 3.097l.068.182s-.022 0-.022.022a.264.264 0 0 0-.022.228c.091.228.205.455.364.66v.045a.396.396 0 0 1 .091.114.81.81 0 0 1 .159.228c.023.022.046.045.046.068 0 0 .022 0 .022.022a.582.582 0 0 1 .023.342.38.38 0 0 1-.205.25c-.068.022-.114.045-.183.045a.511.511 0 0 1-.433-.273c-.022 0-.022-.022-.022-.022-.022-.023-.022-.045-.046-.068-.045-.068-.068-.159-.091-.25l-.046-.137v-.022a3.816 3.816 0 0 0-.296-.706.353.353 0 0 0-.182-.137c0-.023 0-.023-.023-.023l-.091-.159c-.228.068-.479.159-.729.205-.41.114-.82.159-1.229.159a5.368 5.368 0 0 1-1.981-.364l-.091.182c0 .023 0 .023-.023.023a.35.35 0 0 0-.182.137c-.114.228-.228.455-.296.706l-.045.137c-.023.091-.068.159-.091.25-.022.023-.045.045-.045.068-.023 0-.023.022-.023.022a.508.508 0 0 1-.433.273.434.434 0 0 1-.159-.045.469.469 0 0 1-.182-.615c.023 0 .023-.023.023-.023.022-.023.022-.045.045-.068.068-.091.114-.182.159-.228s.068-.068.091-.114v-.023a3.73 3.73 0 0 0 .364-.66.268.268 0 0 0-.023-.228s-.022 0-.022-.022l.114-.16a3.578 3.578 0 0 1-.615-.41 5.493 5.493 0 0 1-1.867-2.664l-.205.022s0-.022-.023-.022a.256.256 0 0 0-.182-.068h-.022a4.015 4.015 0 0 0-.751.205h-.024c-.045 0-.091.023-.137.046-.068.022-.159.068-.25.091-.022 0-.091-.022-.091 0 0 .023 0 .023-.023.023-.045.023-.068.023-.114.023a.424.424 0 0 1-.456-.319.445.445 0 0 1 .364-.524c.023-.023.023-.023.046-.023.045 0 .068-.022.091-.022.091 0 .182-.023.273-.023.045-.022.091-.022.137-.022a4.2 4.2 0 0 0 .774-.137c.068-.046.137-.091.16-.16 0 0 .022 0 .022-.022l.182-.046c-.205-1.298.091-2.618.797-3.734.022-.045.045-.068.068-.114l-.131-.132a.106.106 0 0 1-.004.019v-.023l.004.004c.01-.065-.031-.145-.072-.186-.182-.182-.41-.319-.638-.455l-.136-.069a2.587 2.587 0 0 1-.251-.136c-.022 0-.068-.045-.068-.045s0-.023-.022-.023a.49.49 0 0 1-.092-.639c.068-.114.182-.159.319-.159a.54.54 0 0 1 .319.114l.023.023c.022.022.045.022.068.045.068.069.114.137.182.205.023.022.068.045.091.091.159.182.364.364.569.524.045.022.091.045.137.045.045 0 .068-.023.091-.023h.023l.137.091a5.426 5.426 0 0 1 2.801-1.594c.273-.046.523-.091.774-.114l.023-.182v-.045c.068-.045.091-.114.114-.182 0-.273 0-.524-.045-.774v-.023c0-.045 0-.091-.023-.137a1.129 1.129 0 0 1-.045-.273v-.113c0-.114.045-.228.137-.319.114-.114.25-.182.387-.159a.45.45 0 0 1 .387.478v.137c-.023.091-.023.182-.045.273 0 .045-.023.091-.023.136v.023c-.048.273-.048.524-.048.774.023.068.045.136.114.182v-.023l.023.182a5.84 5.84 0 0 1 2.96 1.184c.183.182.387.364.569.546l.183-.114h.022c.022.023.068.023.091.023.046 0 .091-.023.137-.045.205-.137.41-.319.569-.501.022-.023.068-.046.091-.091.046-.068.114-.137.183-.205.022 0 .045-.022.068-.045l.022-.023a.546.546 0 0 1 .318-.114c.114 0 .251.068.319.16.159.205.113.478-.091.637 0 .023.022.023 0 .046-.023.022-.046.022-.068.045-.092.045-.16.091-.251.137l-.137.068a4.104 4.104 0 0 0-.638.455c-.045.046-.068.137-.068.205v.023l-.136.137c.364.569.638 1.207.797 1.867.137.66.182 1.343.091 2.003l.182.046a.278.278 0 0 0 .16.159c.25.068.523.114.773.137h.023a.297.297 0 0 0 .137.022c.091 0 .182 0 .272.023.046 0 .092 0 .092.023 0 .022.022.022.045.022a.537.537 0 0 1 .41.479.49.49 0 0 1-.453.32z" /><path d="M12.085 14.718a.352.352 0 0 0-.455.091l-1.116 2.027c.456.136.957.228 1.435.228.341 0 .66-.045.979-.114.159-.045.296-.068.433-.091l-1.093-1.981c-.069-.069-.115-.115-.183-.16zm3.644-4.441-1.708 1.548a.36.36 0 0 0-.091.16c-.046.205.068.41.273.455l2.163.615a4.375 4.375 0 0 0-.092-1.435 4.63 4.63 0 0 0-.545-1.343zm-2.073 3.484a.371.371 0 0 0-.205.433l.889 2.141a4.366 4.366 0 0 0 1.366-1.366c.182-.25.318-.547.433-.865l-2.277-.387a.634.634 0 0 0-.206.044z" /></svg>
   }
}