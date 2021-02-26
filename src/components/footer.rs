use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct FooterLinkListProps {
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub children: ChildrenWithProps<FooterLinkListItem>,
}

#[derive(Properties, Clone)]
pub struct FooterLinkListItemProps {
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub url: String,
}

pub struct Footer;
pub struct FooterLinkListItem {
    props: FooterLinkListItemProps,
}
pub struct FooterLinkList {
    props: FooterLinkListProps,
}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <footer class="Footer">
                <div class="FooterLinkLists">
                    <FooterLinkList title="技術関連">
                        <FooterLinkListItem url="https://github.com/InkoHX" text="GitHub" />
                        <FooterLinkListItem url="https://gitlab.com/InkoHX" text="GitLab" />
                        <FooterLinkListItem url="https://qiita.com/InkoHX" text="Qiita" />
                        <FooterLinkListItem url="https://zenn.dev/inkohx" text="Zenn" />
                        <FooterLinkListItem url="https://inkohx.dev" text="Blog" />
                    </FooterLinkList>
                    <FooterLinkList title="ソーシャルメディア">
                        <FooterLinkListItem url="https://twitter.com/InkoHX" text="Twitter" />
                    </FooterLinkList>
                    <FooterLinkList title="ゲーム関連">
                        <FooterLinkListItem url="https://steamcommunity.com/id/InkoHX/" text="Steam" />
                    </FooterLinkList>
                    <FooterLinkList title="コミュニティ">
                        <FooterLinkListItem url="https://discordjs-japan.org" text="Discord.js Japan User Group" />
                    </FooterLinkList>
                </div>
                <small class="Copyright">{ "© 2021 InkoHX" }</small>
            </footer>
        }
    }
}

impl Component for FooterLinkList {
    type Message = ();

    type Properties = FooterLinkListProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let title = self.props.title.as_str();
        let children = self.props.children.iter();

        html! {
            <div class="FooterLinkList">
                <p class="FooterLinkListTitle">{ title }</p>
                <ul class="FooterLinkListItems">
                    { for children }
                </ul>
            </div>
        }
    }
}

impl Component for FooterLinkListItem {
    type Message = ();

    type Properties = FooterLinkListItemProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let text = self.props.text.as_str();
        let url = self.props.url.as_str();

        html! {
            <li class="FooterLinkListItem">
                <a class="FooterLink" href={ url }>{ text }</a>
            </li>
        }
    }
}
