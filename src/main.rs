mod models;
mod numerology;

use models::{LogEntry, Article}; // BbsPostを削除
use web_sys::HtmlInputElement;
use yew::prelude::*;
use web_sys::wasm_bindgen::JsCast;

// アプリケーションの状態モード
#[derive(PartialEq, Clone, Copy)]
enum AppMode {
    Home,
    Terminal,
}

enum Msg {
    Input(String),
    Submit,
    FocusInput,
    OpenArticle(usize),
    BackToHome,
    // サイドバー等のリンク用（今回はダミー）
    NoOp,
}

struct App {
    input_value: String,
    logs: Vec<LogEntry>,
    articles: Vec<Article>,
    mode: AppMode,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let articles = vec![
            Article {
                id: 1,
                title: "数秘術概論：数字に宿る力".to_string(),
                date: "2026-01-12".to_string(),
                content: "数秘術（Numerology）は、西洋占星術やタロットと並ぶ三大占術の一つです。".to_string(),
            },
            Article {
                id: 2,
                title: "RustとWebAssemblyの魔術".to_string(),
                date: "2026-01-13".to_string(),
                content: "メモリ安全性と高速な実行速度は、まるで古代のルーン文字を刻むかのように堅牢なシステムを構築します。".to_string(),
            },
            Article {
                id: 3,
                title: "Occult Code 開発日誌".to_string(),
                date: "2026-01-14".to_string(),
                content: "このサイトはレトロフューチャーな美学に基づいて設計されています。".to_string(),
            },
            Article {
                id: 4,
                title: "カバラの生命の樹について".to_string(),
                date: "2026-01-15".to_string(),
                content: "セフィロトの樹は宇宙の法則を表す図形であり、我々の精神構造の地図でもあります。".to_string(),
            },
        ];

        Self {
            input_value: String::new(),
            logs: Vec::new(),
            articles,
            mode: AppMode::Home,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Input(val) => {
                self.input_value = val;
                true
            }
            Msg::Submit => {
                let input = self.input_value.clone();
                self.input_value.clear();
                self.mode = AppMode::Terminal;
                self.process_shell(&input);
                true
            }
            Msg::FocusInput => {
                let doc = web_sys::window().unwrap().document().unwrap();
                if let Some(el) = doc.get_element_by_id("cmd-input") {
                    let input: HtmlInputElement = el.dyn_into().unwrap();
                    let _ = input.focus();
                }
                false
            }
            Msg::OpenArticle(id) => {
                self.mode = AppMode::Terminal;
                self.logs.clear();
                let cmd = format!("read {}", id);
                self.process_shell(&cmd);
                true
            }
            Msg::BackToHome => {
                self.mode = AppMode::Home;
                self.logs.clear();
                true
            }
            Msg::NoOp => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::Input(input.value())
        });
        let onkeydown = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" { Some(Msg::Submit) } else { None }
        });
        let onclick = ctx.link().callback(|_| Msg::FocusInput);
        let go_home = ctx.link().callback(|_| Msg::BackToHome);

        let css = r#"
            /* 基本スタイル */
            .retro-glow { text-shadow: 0 0 1px rgba(51, 255, 0, 0.6); }
            
            /* レイアウト用コンテナ */
            .site-header {
                border-bottom: 2px solid #33ff00;
                padding: 15px 20px;
                background: rgba(0, 20, 0, 0.9);
                display: flex;
                justify-content: space-between;
                align-items: center;
                flex-shrink: 0;
            }
            .main-container {
                display: flex;
                flex: 1;
                overflow: hidden; /* 内部スクロールのため */
                position: relative;
            }
            .content-area {
                flex: 3; /* メインコンテンツの比率 */
                overflow-y: auto;
                padding: 20px;
                border-right: 1px dashed rgba(51, 255, 0, 0.3);
            }
            .sidebar {
                flex: 1; /* サイドバーの比率 */
                padding: 20px;
                background: rgba(0, 10, 0, 0.4);
                overflow-y: auto;
                display: none; /* スマホ等は非表示にするレスポンシブ対応用（今回は簡易的にMD以上で表示想定） */
            }
            @media (min-width: 768px) {
                .sidebar { display: block; }
            }

            .site-footer {
                border-top: 1px solid #33ff00;
                padding: 10px 20px;
                background: rgba(0, 20, 0, 0.9);
                font-size: 0.8em;
                text-align: center;
                color: rgba(51, 255, 0, 0.6);
                flex-shrink: 0;
            }

            /* カードグリッド */
            .article-grid {
                display: grid;
                grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
                gap: 20px;
            }
            .article-card {
                border: 1px solid #33ff00;
                background: rgba(0, 20, 0, 0.6);
                padding: 15px;
                cursor: pointer;
                transition: all 0.2s;
            }
            .article-card:hover {
                background: rgba(51, 255, 0, 0.1);
                box-shadow: 0 0 8px rgba(51, 255, 0, 0.3);
                transform: translateY(-2px);
            }
            
            /* サイドバーパーツ */
            .sidebar-section { margin-bottom: 30px; }
            .sidebar-title { font-weight: bold; border-bottom: 1px solid #33ff00; margin-bottom: 10px; padding-bottom: 5px; }
            .sidebar-link { display: block; padding: 5px 0; color: #88aa88; cursor: pointer; text-decoration: none; }
            .sidebar-link:hover { color: #33ff00; text-decoration: underline; }

            /* CRTエフェクト */
            .scanlines {
                background: linear-gradient(to bottom, rgba(255,255,255,0), rgba(255,255,255,0) 50%, rgba(0,0,0,0.1) 50%, rgba(0,0,0,0.1));
                background-size: 100% 3px; pointer-events: none;
            }
            .vignette {
                background: radial-gradient(circle, rgba(0,0,0,0) 60%, rgba(0,0,0,0.5) 100%);
                pointer-events: none;
            }
        "#;

        html! {
            <>
                <style>{css}</style>
                <div class="relative w-full h-screen font-mono flex flex-col bg-[#0d1117] text-[#33ff00] overflow-hidden">
                    
                    // --- ヘッダー ---
                    <header class="site-header z-10 retro-glow">
                        <div class="flex items-center gap-4">
                            // 【修正点】ここで go_home をクローンして使用
                            <h1 class="text-2xl font-bold cursor-pointer tracking-wider" onclick={go_home.clone()}>{"OCCULT CODE"}</h1>
                            <nav class="hidden md:flex gap-4 text-sm">
                                // 【修正点】ここは最後の使用なのでそのままでOK
                                <span class="cursor-pointer hover:underline" onclick={go_home}>{"[HOME]"}</span>
                                <span class="cursor-pointer hover:underline">{"[ABOUT]"}</span>
                                <span class="cursor-pointer hover:underline">{"[TOOLS]"}</span>
                            </nav>
                        </div>
                        <div class="text-xs border border-[#33ff00] px-2 py-1 rounded">{"SYS: ONLINE"}</div>
                    </header>

                    // --- メインコンテナ (2カラム) ---
                    <div class="main-container z-10 retro-glow" onclick={onclick}>
                        
                        // 左カラム：メインコンテンツ
                        <main class="content-area">
                            if self.mode == AppMode::Home {
                                <div class="mb-6 p-4 border border-dashed border-[#33ff00]/50 bg-[#33ff00]/5">
                                    <h2 class="text-lg font-bold mb-2">{">> WELCOME TO THE UNDERGROUND"}</h2>
                                    <p class="text-sm opacity-80">{"ここはデジタルとオカルトが交差する場所。数秘術、暗号、そして失われた技術。最新のアーカイブにアクセスするには以下のカードを選択するか、端末にコマンドを入力せよ。"}</p>
                                </div>

                                <div class="article-grid">
                                    { for self.articles.iter().map(|a| {
                                        let id = a.id;
                                        let click_article = ctx.link().callback(move |_| Msg::OpenArticle(id));
                                        html! {
                                            <div class="article-card" onclick={click_article}>
                                                <div class="text-xs text-[#33ff00]/60 mb-1">{ format!("LOG_ID: 00{}", a.id) }</div>
                                                <div class="font-bold text-lg mb-2 border-b border-[#33ff00]/30 pb-1">{ &a.title }</div>
                                                <div class="text-xs text-[#33ff00]/80">{ &a.date }</div>
                                            </div>
                                        }
                                    }) }
                                </div>
                            } else {
                                // ターミナルモード
                                <div id="terminal" class="min-h-full">
                                    { for self.logs.iter().map(|log| self.render_log(log)) }
                                </div>
                            }
                        </main>

                        // 右カラム：サイドバー
                        <aside class="sidebar">
                            <div class="sidebar-section">
                                <h3 class="sidebar-title">{"RECOMMENDED"}</h3>
                                <a class="sidebar-link">{"- 初心者のための数秘術"}</a>
                                <a class="sidebar-link">{"- ターミナルコマンド一覧"}</a>
                                <a class="sidebar-link">{"- WebAssemblyとは"}</a>
                            </div>
                            <div class="sidebar-section">
                                <h3 class="sidebar-title">{"TAGS"}</h3>
                                <div class="flex flex-wrap gap-2 text-xs">
                                    <span class="border border-[#33ff00] px-1">{"RUST"}</span>
                                    <span class="border border-[#33ff00] px-1">{"OCCULT"}</span>
                                    <span class="border border-[#33ff00] px-1">{"CODE"}</span>
                                    <span class="border border-[#33ff00] px-1">{"NUMBERS"}</span>
                                </div>
                            </div>
                            <div class="sidebar-section">
                                <h3 class="sidebar-title">{"STATUS"}</h3>
                                <div class="text-xs font-mono">
                                    <div>{"MEM: 64KB OK"}</div>
                                    <div>{"NET: CONNECTED"}</div>
                                    <div>{"SEC: LOW"}</div>
                                </div>
                            </div>
                        </aside>
                    </div>

                    // --- フッター (コマンドライン含む) ---
                    <footer class="site-footer z-10 flex flex-col gap-2">
                        <div class="w-full flex items-center bg-black/50 p-2 border border-[#33ff00]/30 rounded">
                            <span class="mr-2 text-[#33ff00] font-bold text-sm">{"root@occult-code:~#"}</span>
                            <input type="text" id="cmd-input"
                                class="w-full bg-transparent border-none outline-none text-[#33ff00] font-bold"
                                value={self.input_value.clone()}
                                placeholder="Enter command..."
                                oninput={oninput} onkeydown={onkeydown} autofocus=true />
                        </div>
                        <div class="text-xs opacity-50">
                            {"(C) 2026 OCCULT CODE PROJECT. ALL RIGHTS RESERVED."}
                        </div>
                    </footer>

                    // --- エフェクトレイヤー ---
                    <div class="absolute inset-0 z-20 scanlines w-full h-full opacity-50 pointer-events-none"></div>
                    <div class="absolute inset-0 z-30 vignette w-full h-full pointer-events-none"></div>
                </div>
            </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first: bool) {
        if first {
            let doc = web_sys::window().unwrap().document().unwrap();
            if let Some(el) = doc.get_element_by_id("cmd-input") {
                let input: HtmlInputElement = el.dyn_into().unwrap();
                let _ = input.focus();
            }
        }
    }
}

impl App {
    fn render_log(&self, log: &LogEntry) -> Html {
        if log.is_html {
            let div = web_sys::window().unwrap().document().unwrap().create_element("div").unwrap();
            div.set_inner_html(&log.text);
            Html::VRef(div.into())
        } else {
            html! { <div class={&log.class}>{ &log.text }</div> }
        }
    }

    fn process_shell(&mut self, input: &str) {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts.get(0).map(|s| *s).unwrap_or("");
        self.log_text(&format!("root@occult-code:~# {}", input), "text-gray-500", "INPUT", "");

        match command {
            "help" => {
                self.log_text("Available commands:", "text-yellow-400", "SYSTEM", "");
                self.log_text("  home              : Return to Top Page", "", "SYSTEM", "");
                self.log_text("  divine <YYYYMMDD> : Calculate Life Path Number", "", "SYSTEM", "");
                self.log_text("  ls                : List articles", "", "SYSTEM", "");
                self.log_text("  clear             : Clear terminal", "", "SYSTEM", "");
            }
            "home" | "exit" => {
                self.mode = AppMode::Home;
                self.logs.clear();
            }
            "ls" => {
                let list = self.articles.clone();
                for a in list {
                    self.log_text(&format!("ID: {} | {} ({})", a.id, a.title, a.date), "", "RESULT", "");
                }
            }
            "clear" => self.logs.clear(),
            "read" => {
                 if let Some(id_str) = parts.get(1) {
                    if let Ok(id) = id_str.parse::<usize>() {
                        if let Some(a) = self.articles.iter().find(|x| x.id == id) {
                             let html_content = format!(
                                r#"
                                <div style="margin-top: 20px; border: 1px solid #33ff00; padding: 20px; background: rgba(0, 20, 0, 0.8);">
                                    <h2 style="font-size: 1.8em; border-bottom: 2px solid #33ff00; margin-bottom: 15px;">{}</h2>
                                    <div style="font-size: 0.8em; color: #88aa88; margin-bottom: 20px;">ACCESS DATE: {}</div>
                                    <div style="line-height: 1.8; color: #ddffdd; white-space: pre-wrap;">{}</div>
                                    <div style="margin-top: 30px; border-top: 1px dashed #33ff00; padding-top: 10px; text-align: right;">
                                        <span style="cursor: pointer; color: #33ff00; text-decoration: underline;" onclick="window.location.reload()">[ CLOSE FILE ]</span>
                                    </div>
                                </div>
                                "#, a.title, a.date, a.content
                            );
                            self.log_html(&html_content, "RESULT", "Article Content");
                        } else {
                            self.log_text("Error: Article ID not found.", "text-red-500", "ERROR", "");
                        }
                    }
                }
            }
            "divine" => {
                if let Some(date_str) = parts.get(1) {
                    if let Some(num) = numerology::calculate_life_path(date_str) {
                        let meaning = numerology::get_meaning(num);
                        let html_card = format!(
                            r#"
                            <div class="retro-glow" style="border: 2px solid #33ff00; padding: 20px; margin: 10px 0; max-width: 600px; background: rgba(0, 10, 0, 0.9);">
                                <h3 style="font-size: 1.5em; border-bottom: 1px solid #33ff00; margin-bottom: 10px;">
                                    ANALYSIS REPORT: TARGET [{}]
                                </h3>
                                <div style="display: flex; gap: 20px; align-items: center;">
                                    <div style="font-size: 4em; font-weight: bold; color: #33ff00;">
                                        {}
                                    </div>
                                    <div>
                                        <p style="margin: 0; font-size: 1.1em;">CODE: LIFE_PATH</p>
                                        <p style="margin: 5px 0 0 0; color: #ccffcc;">{}</p>
                                    </div>
                                </div>
                                <div style="margin-top: 15px; font-size: 0.8em; color: #88aa88;">
                                    TIMESTAMP: {} <br>
                                    STATUS: VERIFIED
                                </div>
                            </div>
                            "#, date_str, num, meaning, "2026-01-13"
                        );
                        self.log_html(&html_card, "RESULT", "Numerology Result");
                    } else {
                        self.log_text("Error: Invalid date format. Use YYYYMMDD.", "text-red-500", "ERROR", "");
                    }
                } else {
                    self.log_text("Usage: divine <YYYYMMDD>", "text-yellow-500", "ERROR", "");
                }
            }
            "" => {}
            _ => self.log_text("Command not found. Type 'help'.", "text-red-500", "ERROR", ""),
        }
    }

    fn log_text(&mut self, text: &str, class: &str, category: &str, description: &str) {
        self.logs.push(LogEntry {
            text: text.to_string(),
            class: class.to_string(),
            is_html: false,
            category: category.to_string(),
            description: description.to_string(),
        });
    }

    fn log_html(&mut self, html: &str, category: &str, description: &str) {
        self.logs.push(LogEntry {
            text: html.to_string(),
            class: "".to_string(),
            is_html: true,
            category: category.to_string(),
            description: description.to_string(),
        });
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}