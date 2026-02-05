use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use serde::Deserialize;
use reqwasm::http::Request;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/logs")]
    Logs,
    #[at("/admin")]
    Admin,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, PartialEq, Deserialize)]
struct Post {
    id: usize,
    title: String,
    date: String,
    url: String,
}

// ★追加: JSONの「箱」の定義
#[derive(Clone, PartialEq, Deserialize)]
struct PostWrapper {
    posts: Vec<Post>,
}

#[derive(Clone, PartialEq)]
struct CipherResult {
    name: String,
    score: u32,
    breakdown: String,
}

struct GematriaDecoder {
    input_value: String,
    results: Vec<CipherResult>,
}

enum DecoderMsg {
    UpdateInput(String),
}

impl Component for GematriaDecoder {
    type Message = DecoderMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_value: String::new(),
            results: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DecoderMsg::UpdateInput(val) => {
                if val.is_empty() {
                    self.input_value = val;
                    self.results.clear();
                    return true;
                }

                let mut std_sum = 0;
                let mut rev_sum = 0;
                let mut red_sum = 0;
                let mut std_parts = Vec::new();
                let mut rev_parts = Vec::new();

                for c in val.to_ascii_uppercase().chars() {
                    if c.is_ascii_alphabetic() {
                        let base_num = c as u32 - 64;
                        std_sum += base_num;
                        std_parts.push(base_num.to_string());
                        let rev_num = 27 - base_num;
                        rev_sum += rev_num;
                        rev_parts.push(rev_num.to_string());
                        let red_num = (base_num - 1) % 9 + 1;
                        red_sum += red_num;
                    }
                }

                self.results = vec![
                    CipherResult { name: "Standard".to_string(), score: std_sum, breakdown: std_parts.join("+") },
                    CipherResult { name: "Reverse".to_string(), score: rev_sum, breakdown: rev_parts.join("+") },
                    CipherResult { name: "Reduction".to_string(), score: red_sum, breakdown: "Reduced".to_string() },
                ];
                self.input_value = val;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            DecoderMsg::UpdateInput(input.value())
        });

        html! {
            <div>
                <section class="input-group">
                    <label>{ "> TARGET_IDENTIFIER: " }</label>
                    <input type="text" class="cmd-input" value={self.input_value.clone()} 
                           oninput={oninput} placeholder="Enter keyword..." autofocus=true />
                </section>
                if !self.results.is_empty() {
                    <section class="cipher-grid">
                        { for self.results.iter().map(|res| html! {
                            <div class="cipher-card">
                                <span class="cipher-label">{ &res.name }</span>
                                <span class="cipher-breakdown">{ &res.breakdown }</span>
                                <span class="cipher-value">{ res.score }</span>
                            </div>
                        }) }
                    </section>
                }
            </div>
        }
    }
}

#[function_component(Logs)]
fn logs() -> Html {
    let posts = use_state(|| Vec::<Post>::new());
    
    {
        let posts = posts.clone();
        use_effect_with((), move |_| {
            let posts = posts.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // ★修正: JSONを PostWrapper 型として受け取る
                let fetched_wrapper: PostWrapper = Request::get("/posts.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                // 中身の .posts をセットする
                posts.set(fetched_wrapper.posts);
            });
            || ()
        });
    }

    html! {
        <section>
            <h2>{ "> SYSTEM_LOGS" }</h2>
            <p>{ "Accessing archived protocols..." }</p>
            
            <ul style="margin-top: 1rem; list-style: none; padding: 0;">
                { for posts.iter().map(|post| html! {
                    <li style="margin-bottom: 1.5rem; border-left: 2px solid #333; padding-left: 10px;">
                        <span style="color: #888; font-size: 0.8rem;">{ &post.date }</span><br/>
                        <a href={post.url.clone()} style="font-size: 1.1rem;">{ &post.title }</a>
                    </li>
                }) }
            </ul>

            if posts.is_empty() {
                <p style="color: #555;">{ "Scanning database..." }</p>
            }
        </section>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <GematriaDecoder /> },
        Route::Logs => html! { <Logs /> },
        Route::Admin => html! { <p>{ "Redirecting..." }</p> },
        Route::NotFound => html! { <h1>{ "404: SIGNAL LOST" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="terminal-wrapper">
                <header>
                    <h1 class="cursor-blink">{ "OCCULT CODE SYSTEM" }</h1>
                    <p class="status-line">
                        { "STATUS: ONLINE" } <span class="separator">{ "|" }</span>
                        { "USER: 418-User" } <span class="separator">{ "|" }</span>
                        { "NAV: ROUTING_ENABLED" }
                    </p>
                </header>
                <nav>
                    <ul>
                        <li><Link<Route> to={Route::Home}>{ "[H]ome" }</Link<Route>></li>
                        <li><Link<Route> to={Route::Logs}>{ "[L]ogs" }</Link<Route>></li>
                        <li><a href="/admin/">{ "[S]ystem_Admin" }</a></li>
                    </ul>
                </nav>
                <hr class="dashed-line" />
                <main>
                    <Switch<Route> render={switch} />
                </main>
                <footer>
                    <p>{ "END OF LINE." }</p>
                    <p>{ "© 2026 OCCULT CODE" }</p>
                </footer>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}