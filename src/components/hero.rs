use yew::prelude::*;

pub struct Hero;

impl Component for Hero {
    type Message = ();

    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section class="Hero">
                <div class="HeroText">
                    <h1>{ "InkoHX" }</h1>
                    <p>{ "JavaScriptとTypeScriptをメインに扱うインコ大好き学生Coderです。" }</p>
                    <p>
                        { "何か聞きたいことがあれば、" }
                        <a class="TwitterLink" href="https://twitter.com/InkoHX">{ "Twitter" }</a>
                        { "またはDiscord" }
                        <code class="DiscordTag">{ "InkoHX#7777" }</code>
                        { "に連絡を頂ければ結構早くレスポンスが来るかもしれません。" }
                    </p>
                </div>
                <div class="HeroFloatIconContainer">
                    <div class="HeroFloatIcons">
                        <img class="HeroFloatIcon Git" src="/icons/git.svg" alt="Git" />
                        <img class="HeroFloatIcon GitHub" src="/icons/github.svg" alt="GitHub" />
                        <img class="HeroFloatIcon JavaScript" src="/icons/js.svg" alt="JavaScript" />
                        <img class="HeroFloatIcon NodeJS" src="/icons/nodejs.svg" alt="Node.js" />
                        <img class="HeroFloatIcon Rust" src="/icons/rust.svg" alt="Rust" />
                        <img class="HeroFloatIcon Docker" src="/icons/docker.svg" alt="Docker" />
                        <img class="HeroFloatIcon HTML5" src="/icons/html5.svg" alt="HTML5" />
                        <img class="HeroFloatIcon React" src="/icons/react.svg" alt="React.js" />
                        <img class="HeroFloatIcon VueJS" src="/icons/vuejs.svg" alt="Vue.js" />
                        <img class="HeroFloatIcon Windows10" src="/icons/windows10.svg" alt="Windows10" />
                    </div>
                </div>
            </section>
        }
    }
}
