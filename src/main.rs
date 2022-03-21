use yew::prelude::*;

enum AppMessage {
    Inc,
    Dec
}

struct StructComponent {
    counter: i64,
}

impl Component for StructComponent {
    type Message = AppMessage; // Message dispached
    type Properties = (); // Props

    fn create(_ctx: &Context<Self>) -> Self {
        Self { counter: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::Inc => {
                self.counter += 1;
            }
            AppMessage::Dec => {
                self.counter -= 1;
            }
        }
        true // re-renders the component
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <span>
                <span>
                    <p>{ "count: " }</p>
                    <p>{ self.counter }</p>
                </span>
                <span>
                    <button onclick={link.callback(|_| AppMessage::Inc)}>{ "+" }</button>
                    <button onclick={link.callback(|_| AppMessage::Dec)}>{ "-" }</button>
                </span>
            </span>
        }
    }
}

fn main() {
    yew::start_app::<StructComponent>();
}
