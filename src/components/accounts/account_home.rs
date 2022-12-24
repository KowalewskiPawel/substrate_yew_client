use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{yew::styled_component, Style};
use crate::components::navigation::nav::Nav;
use crate::router::Route;

const STYLE_FILE: &str = include_str!("account_home.css");

#[styled_component(AccountHome)]
pub fn account_home() -> Html {

    let stylesheet = Style::new(STYLE_FILE).unwrap();
    let navigation = use_navigator().unwrap();

    let sign_in_button = {
        let navigator = navigation.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::AddAccounts));
        html! {
            <button type="button" class="btn btn-warning" {onclick}>{"Sign In"}</button>
        }
    };

    html!{
        <>
            <Nav/> 
                <div class={classes!("container", stylesheet)}>
                    <div class="card shadow-lg p-3 mb-5 bg-body rounded home">
                        <div class="d-grid gap-3">
                            {sign_in_button}
                            <button type="button" class="btn btn-warning">{"Generate Account"}</button>                            
                        </div>
                    </div>
                </div>
        </>
    }
}