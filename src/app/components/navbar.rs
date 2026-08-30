use topcoat::{
    Result,
    view::{component, view},
};

#[component]
pub async fn navbar() -> Result {
    let navlinks = vec![("Dashboard", "/dashboard"), ("About", "/about")];

    // Build the menu lists synchronously using a loop
    // let mut mobile_links = Vec::new();
    // let mut desktop_links = Vec::new();

    // for &(name, href) in &navlinks {
    //     mobile_links.push(
    //         view! {
    //             <li><a href=(href)>(name)</a></li>
    //         }
    //         .await?,
    //     );

    //     desktop_links.push(
    //         view! {
    //             <li><a href=(href)>(name)</a></li>
    //         }
    //         .await?,
    //     );
    // }

    let links: Vec<_> = navlinks
        .iter()
        .map(|&(name, href)| {
            view! {
                <li><a href=(href)>"(name)"</a></li>
            }
        })
        .collect::<Result<Vec<_>>>()?; // Bubbles up any rendering errors cleanly using ?

    view! {
        <nav>
            <div class="navbar bg-base-100 shadow-sm">
                <div class="navbar-start">
                    <div class="dropdown">
                        <div tabindex="0" role="button" class="btn btn-ghost lg:hidden">
                            hamburger_menu()
                        </div>
                        <ul
                            tabindex="-1"
                            class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow"
                        >
                            // Render mobile menu links directly
                            // (mobile_links)
                            (links.clone())
                        </ul>
                    </div>
                    <a class="btn btn-ghost text-xl" href="/">"Home"</a>
                </div>
                <div class="navbar-center hidden lg:flex">
                    <ul class="menu menu-horizontal px-1">
                        // Render desktop menu links directly
                        // (desktop_links)
                        (links.clone())
                    </ul>
                </div>
                <div class="navbar-end"><a class="btn">"Button"</a></div>
            </div>
        </nav>
    }
}

#[component]
pub async fn hamburger_menu() -> Result {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-5 w-5"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 6h16M4 12h8m-8 6h16"
            />
        </svg>
    }
}
