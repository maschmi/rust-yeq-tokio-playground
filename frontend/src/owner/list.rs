use anyhow::{anyhow, Error};
use common::OwnerResponse;
use serde_json::Value;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};
use yew::prelude::*;

pub struct List {
    owners: Option<Vec<OwnerResponse>>,
    loading: bool,
    error: bool,
}

pub enum ListMsg {
    MakeReq,
    Resp(Vec<OwnerResponse>),
    Error,
}

impl List {
    fn render_list(&self) -> Html {
        if let Some(t) = &self.owners {
            html! {
                <div class={classes!("list")}>
                    { t.iter().map(|name| self.view_owner(name)).collect::<Html>() }
                </div>
            }
        } else {
            html! {
                <div class={classes!("loading")}>{"loading..."}</div>
            }
        }
    }

    fn view_owner(&self, owner: &OwnerResponse) -> Html {
        html! {
            <div class={classes!("list-item")}>
                { &owner.name }
            </div>
        }
    }
}

pub async fn fetch_data() -> Result<Vec<OwnerResponse>, Error> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    let url = "http://localhost:8001/owner";
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
    let st_json = js_sys::JSON::stringify(&json).unwrap();
    let v: Value = serde_json::from_str(st_json.as_string().unwrap().as_str()).unwrap();
    serde_json::from_value(v).map_err(|e| anyhow!(e))
}


impl Component for List {
    type Message = ListMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(ListMsg::MakeReq);
        Self {
            owners: None,
            loading: true,
            error: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ListMsg::MakeReq => {
                ctx.link().send_future(async {
                    match fetch_data().await {
                        Ok(data) => ListMsg::Resp(data),
                        Err(_) => ListMsg::Error
                    }
                });
                false
            }
            ListMsg::Resp(resp) => {
                self.owners = Some(resp);
                self.loading = true;
                self.error = false;
                true
            }
            ListMsg::Error => {
                self.loading = false;
                self.error = true;
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                { self.render_list() }
            </div>
        }
    }
}
