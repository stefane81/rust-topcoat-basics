use topcoat::{
    Result,
    view::{component, view},
};

#[component]
pub async fn navbar_links() -> Result {
    let navlinks = vec![
        ("Home", "/"),
        ("Dashboard", "/dashboard"),
        ("About", "/about"),
    ];
    view! {
            for (name,href) in navlinks {
                    <li><a href=(href)>(name)</a></li>
            }
    }
}

#[component]
pub async fn navbar() -> Result {
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
                        navbar_links()
                        </ul>
                    </div>
                    <a class="btn btn-ghost text-xl" href="/">"Home"</a>
                </div>
                <div class="navbar-center hidden lg:flex">
                    <ul class="menu menu-horizontal px-1">
                        navbar_links()
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
