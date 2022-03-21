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

#[function_component(FunctionComponent)]
fn function_component() -> Html {
    let counter = use_state(|| 0i64);

    let get_butn_click_handler = |amount: i64| { 
        let counter = counter.clone();
        
        return Callback::from(move |_| {
            counter.set(*counter + amount);
        });
    };

    html! {
        <span>
            <span>
                <p>{ "count: " }</p>
                <p>{ *counter }</p>
            </span>
            <span>
                <button onclick={get_butn_click_handler(1)}>{ "+" }</button>
                <button onclick={get_butn_click_handler(-1)}>{ "-" }</button>
            </span>
        </span>
    }
}

#[function_component(AppComponent)]
fn app_component() -> Html {
    html! {
        <span>
            <p>{ "Application" }</p>
            <span>
                <StructComponent />
                <FunctionComponent />
            </span>
        </span>
    }
}

fn main() {
    yew::start_app::<AppComponent>();
}
