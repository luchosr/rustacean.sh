use leptos::{component, create_rw_signal, spawn_local, view, IntoView, SignalGet, SignalSet};

use crate::services::Services;

#[component]
pub fn GitHubStars() -> impl IntoView {
    let stars = create_rw_signal::<u32>(0);

    spawn_local(async move {
        let services = Services::new();

        match services.github().stars().await {
            Ok(gh_stars) => {
                stars.set(gh_stars);
            }
            Err(err) => {
                leptos::logging::error!("{err}");
                // error.set(Some(err.to_string()));
            }
        }
    });

    view! {
        <div id="github-stars" class="flex items-center gap-x-1 md:gap-x-2 ms-auto py-1 md:ps-6 md:order-3 md:col-span-3">
            <a href="https://github.com/rustacean-sh/rustacean.sh"  target="_blank" class="py-2 px-3 inline-flex items-center gap-x-2 text-sm font-medium rounded-md bg-white border border-gray-200 text-black hover:bg-gray-100 focus:outline-none focus:bg-gray-100 disabled:opacity-50 disabled:pointer-events-none ">
            <figure >
                <img  height="15" width="15" src={"../assets/images/star-svgrepo-com.svg"} alt={"Star in github"} />
            </figure>
            <span class="border-r border-gray-300 px-1">"Star"</span>
                {move || stars.get()}
            </a>
        </div>
    }
}
