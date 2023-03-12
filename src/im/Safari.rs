#[cfg(feature = "ImSafari")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImSafari")]
/// *This icon requires the feature* `ImSafari` *to be enabled*.
#[component]
pub fn Safari(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M8 0c-4.419 0-8 3.581-8 8s3.581 8 8 8 8-3.581 8-8-3.581-8-8-8zM14.975 7.388l-0.016-0.166c0.003 0.056 0.009 0.109 0.016 0.166zM13.881 4.2l-0.113-0.169c0.037 0.056 0.075 0.112 0.113 0.169zM13.447 3.603l-0.069-0.084c0.025 0.028 0.047 0.056 0.069 0.084zM12.478 2.619l-0.084-0.069c0.031 0.025 0.056 0.047 0.084 0.069zM11.969 2.231l-0.169-0.112c0.056 0.038 0.113 0.075 0.169 0.112zM8.778 1.044l-0.169-0.016c0.056 0.003 0.113 0.009 0.169 0.016zM7.388 1.025l-0.169 0.016c0.056-0.003 0.112-0.009 0.169-0.016zM4.2 2.119l-0.169 0.112c0.056-0.038 0.112-0.075 0.169-0.112zM3.603 2.553l-0.081 0.066c0.028-0.022 0.053-0.044 0.081-0.066zM2.619 3.522l-0.069 0.084c0.025-0.028 0.047-0.056 0.069-0.084zM2.231 4.031l-0.112 0.169c0.038-0.056 0.075-0.112 0.112-0.169zM1.044 7.222l-0.016 0.169c0.003-0.056 0.009-0.112 0.016-0.169zM1.025 8.613l0.016 0.169c-0.003-0.056-0.009-0.113-0.016-0.169zM2.119 11.797l0.112 0.169c-0.038-0.053-0.075-0.109-0.112-0.169zM2.25 11.994l1.247-0.834-0.138-0.209-1.247 0.834c-0.566-0.878-0.938-1.887-1.063-2.975l0.747-0.075-0.025-0.25-0.747 0.075c-0.012-0.144-0.019-0.291-0.022-0.438h1.5v-0.25h-1.5c0.003-0.147 0.009-0.291 0.022-0.438l0.747 0.072 0.025-0.25-0.747-0.072c0.125-1.088 0.5-2.097 1.066-2.975l1.247 0.834 0.138-0.209-1.25-0.828c0.084-0.119 0.169-0.237 0.259-0.35l0.578 0.475 0.159-0.194-0.578-0.475c0.094-0.112 0.194-0.219 0.294-0.325l1.059 1.059 0.178-0.178-1.059-1.059c0.106-0.1 0.212-0.2 0.322-0.294l0.475 0.581 0.194-0.159-0.475-0.578c0.116-0.091 0.231-0.178 0.35-0.263l0.834 1.247 0.209-0.138-0.834-1.247c0.878-0.566 1.888-0.938 2.975-1.063l0.075 0.747 0.25-0.025-0.075-0.747c0.144-0.012 0.291-0.019 0.438-0.022v1.5h0.25v-1.5c0.147 0.003 0.291 0.009 0.438 0.022l-0.072 0.747 0.25 0.025 0.072-0.747c1.088 0.125 2.097 0.5 2.975 1.066l-0.834 1.247 0.209 0.138 0.834-1.247c0.119 0.084 0.238 0.169 0.35 0.259l-0.475 0.578 0.194 0.159 0.475-0.578c0.113 0.094 0.219 0.194 0.325 0.294l-0.4 0.391-5.469 3.647-3.647 5.469-0.391 0.391c-0.1-0.106-0.2-0.213-0.294-0.322l0.578-0.475-0.159-0.194-0.578 0.475c-0.091-0.113-0.175-0.231-0.259-0.35zM2.619 12.478c-0.022-0.028-0.044-0.053-0.066-0.081l0.066 0.081zM3.522 13.381l0.081 0.066c-0.028-0.022-0.053-0.044-0.081-0.066zM4.031 13.766l0.169 0.113c-0.056-0.034-0.112-0.072-0.169-0.113zM7.222 14.956l0.169 0.016c-0.056-0.003-0.112-0.009-0.169-0.016zM8.613 14.975l0.166-0.016c-0.056 0.003-0.109 0.009-0.166 0.016zM11.8 13.881l0.169-0.113c-0.056 0.037-0.113 0.075-0.169 0.113zM12.397 13.447l0.084-0.069c-0.028 0.025-0.056 0.047-0.084 0.069zM12.944 12.956l0.012-0.012c-0.003 0.003-0.009 0.009-0.012 0.012zM13.381 12.478l0.069-0.084c-0.025 0.028-0.047 0.056-0.069 0.084zM13.491 12.344l-0.578-0.475-0.159 0.194 0.578 0.475c-0.094 0.113-0.194 0.219-0.294 0.325l-1.059-1.059-0.178 0.178 1.059 1.059c-0.106 0.1-0.213 0.2-0.322 0.294l-0.475-0.581-0.194 0.159 0.475 0.578c-0.116 0.091-0.231 0.178-0.35 0.262l-0.834-1.247-0.209 0.137 0.834 1.247c-0.878 0.566-1.887 0.938-2.975 1.063l-0.075-0.747-0.25 0.025 0.075 0.747c-0.144 0.012-0.291 0.019-0.438 0.022v-1.5h-0.25v1.5c-0.147-0.003-0.291-0.009-0.438-0.022l0.072-0.747-0.25-0.025-0.072 0.747c-1.088-0.125-2.097-0.5-2.975-1.066l0.834-1.247-0.209-0.137-0.828 1.247c-0.119-0.084-0.237-0.169-0.35-0.259l0.475-0.578-0.194-0.159-0.475 0.578c-0.112-0.094-0.219-0.194-0.325-0.294l0.394-0.391 5.469-3.647 3.647-5.469 0.391-0.391c0.1 0.106 0.2 0.212 0.294 0.322l-0.578 0.475 0.159 0.194 0.578-0.475c0.091 0.116 0.178 0.231 0.262 0.35l-1.247 0.834 0.137 0.209 1.247-0.834c0.566 0.878 0.938 1.888 1.063 2.975l-0.747 0.075 0.025 0.25 0.747-0.075c0.012 0.144 0.019 0.291 0.022 0.438h-1.5v0.25h1.5c-0.003 0.147-0.009 0.291-0.022 0.438l-0.747-0.072-0.025 0.25 0.747 0.072c-0.125 1.088-0.5 2.097-1.066 2.975l-1.247-0.834-0.137 0.209 1.247 0.834c-0.081 0.113-0.169 0.228-0.259 0.344zM14.975 8.609c-0.006 0.056-0.009 0.113-0.016 0.169l0.016-0.169zM13.881 11.8c-0.037 0.056-0.075 0.113-0.113 0.169l0.113-0.169z" /><path fill="#000000" d="M6.758 1.111l0.293 1.471-0.245 0.049-0.293-1.471 0.245-0.049z" /><path fill="#000000" d="M9.245 14.89l-0.293-1.471 0.245-0.049 0.293 1.471-0.245 0.049z" /><path fill="#000000" d="M6.088 1.264l0.218 0.718-0.239 0.073-0.218-0.718 0.239-0.073z" /><path fill="#000000" d="M9.913 14.733l-0.218-0.718 0.239-0.073 0.218 0.718-0.239 0.073z" /><path fill="#000000" d="M5.438 1.486l0.574 1.386-0.231 0.096-0.574-1.386 0.231-0.096z" /><path fill="#000000" d="M10.564 14.515l-0.574-1.386 0.231-0.096 0.574 1.386-0.231 0.096z" /><path fill="#000000" d="M4.588 1.885l0.22-0.118 0.354 0.661-0.22 0.118-0.354-0.661z" /><path fill="#000000" d="M11.408 14.114l-0.22 0.118-0.354-0.661 0.22-0.118 0.354 0.661z" /><path fill="#000000" d="M1.884 4.591l0.662 0.353-0.118 0.221-0.661-0.353 0.118-0.221z" /><path fill="#000000" d="M14.113 11.409l-0.662-0.353 0.118-0.22 0.662 0.353-0.118 0.22z" /><path fill="#000000" d="M2.872 6.010l-1.386-0.574 0.096-0.231 1.386 0.574-0.096 0.231z" /><path fill="#000000" d="M13.13 9.989l1.386 0.574-0.096 0.231-1.386-0.574 0.096-0.231z" /><path fill="#000000" d="M1.337 5.85l0.718 0.218-0.073 0.239-0.718-0.218 0.073-0.239z" /><path fill="#000000" d="M14.661 10.151l-0.718-0.218 0.073-0.239 0.718 0.218-0.073 0.239z" /><path fill="#000000" d="M1.157 6.512l1.471 0.293-0.049 0.245-1.471-0.293 0.049-0.245z" /><path fill="#000000" d="M14.84 9.488l-1.471-0.293 0.049-0.245 1.471 0.293-0.049 0.245z" /><path fill="#000000" d="M1.109 9.243l1.471-0.293 0.049 0.245-1.471 0.293-0.049-0.245z" /><path fill="#000000" d="M14.888 6.757l-1.471 0.293-0.049-0.245 1.471-0.293 0.049 0.245z" /><path fill="#000000" d="M1.265 9.914l0.718-0.218 0.073 0.239-0.718 0.218-0.073-0.239z" /><path fill="#000000" d="M14.734 6.089l-0.718 0.218-0.073-0.239 0.718-0.218 0.073 0.239z" /><path fill="#000000" d="M1.58 10.796l-0.096-0.231 1.386-0.574 0.096 0.231-1.386 0.574z" /><path fill="#000000" d="M14.419 5.207l0.096 0.231-1.386 0.574-0.096-0.231 1.386-0.574z" /><path fill="#000000" d="M1.888 11.41l-0.118-0.22 0.661-0.354 0.118 0.22-0.661 0.354z" /><path fill="#000000" d="M14.116 4.59l0.118 0.22-0.661 0.354-0.118-0.22 0.661-0.354z" /><path fill="#000000" d="M4.811 14.232l-0.22-0.118 0.354-0.661 0.22 0.118-0.354 0.661z" /><path fill="#000000" d="M11.189 1.767l0.22 0.118-0.353 0.661-0.22-0.118 0.353-0.661z" /><path fill="#000000" d="M5.207 14.419l0.574-1.386 0.231 0.096-0.574 1.386-0.231-0.096z" /><path fill="#000000" d="M10.795 1.58l-0.574 1.386-0.231-0.096 0.574-1.386 0.231 0.096z" /><path fill="#000000" d="M6.088 14.735l-0.239-0.073 0.218-0.718 0.239 0.073-0.218 0.718z" /><path fill="#000000" d="M9.912 1.264l0.239 0.073-0.218 0.718-0.239-0.073 0.218-0.718z" /><path fill="#000000" d="M6.757 14.888l-0.245-0.049 0.293-1.471 0.245 0.049-0.293 1.471z" /><path fill="#000000" d="M9.243 1.109l0.245 0.049-0.293 1.471-0.245-0.049 0.293-1.471z" /></svg>
   }
}