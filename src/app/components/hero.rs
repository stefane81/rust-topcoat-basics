use topcoat::{
    Result,
    asset::{Asset, asset},
    view::{component, view},
};

// const : Asset = asset!("assets/hero.jpg");

// const HERO: Asset =
//     asset!("https://img.daisyui.com/images/stock/photo-1507358522600-9f71e620c44e.webp");
// const HEROSTYLE: Asset = format!("background-image: {}", HERO);

#[component]
pub async fn hero() -> Result {
    // let hero_style = "background-image: "(asset!(hero.jpg));
    const HERO: Asset = asset!("assets/hero.jpg");

    // let mut s: String = String::new();
    // "hello".clone_into(&mut s);

    // const HERO_STYLE: String = format!("style=background-image: url({:?});", (HERO));

    // let mut html = String::new();
    // html.push_str(r#"<"background-color: "#);
    // html.push_str(HERO);
    // html.push_str(r#";">"#);

    // println!("{}", html);
    // style="background-image: url(https://img.daisyui.com/images/stock/photo-1507358522600-9f71e620c44e.webp)"
    // style="background-image: url(/target/debug/assets/hero-1a3d8aa1ca907170.jpg)"
    view! {
        <div
            class="hero min-h-screen"
            style="background-image: url(/_topcoat/assets/hero-1a3d8aa1ca907170.jpg);"
            // style="background-image: url(https://img.daisyui.com/images/stock/photo-1507358522600-9f71e620c44e.webp);"
            >
            // style=(hero_style)
             // style="background-image: "(HERO.url)"">
            <div class="hero-overlay"></div>
            <div class="hero-content text-neutral-content text-center">
                <div class="max-w-md">
                    <h1 class="mb-5 text-5xl font-bold">"Hello there"</h1>
                    <p class="mb-5">
                        "Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem
                quasi. In deleniti eaque aut repudiandae et a id nisi."
                    </p>
                    <button class="btn btn-primary">"Get Started"</button>
                </div>
            </div>
        </div>

        <img src=(HERO) style="display:none">
        // <img src=(HERO)>

    }
}
