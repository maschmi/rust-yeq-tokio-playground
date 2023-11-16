use yew::html::Scope;
use yew::prelude::*;
use yew_router::prelude::*;

mod owner; //mod pet;
use owner::list::List;

#[derive(Routable, Clone, Debug, PartialEq)]
pub enum AppRoute {
    #[at("/app/create-owner")]
    CreateOwner,
    #[at("/app/create-pet/:id")]
    CreatePet { id: i32 },
    #[at("/app/:id")]
    Detail { id: i32 },
    #[at("/")]
    Home,
}

struct FullStackApp {
    navbar_active: bool,
}

enum Msg {}

impl Component for FullStackApp {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link())}
            <main>
                <Switch<AppRoute> render={switch} />
            </main>
            </BrowserRouter>
        }
    }
}

impl FullStackApp {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;
        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "PetOwners" }</h1>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<AppRoute> classes={classes!("navbar-item")} to={AppRoute::Home}>
                            { "Home" }
                        </Link<AppRoute>>
                    </div>
                </div>
            </nav>
        }
    }
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => {
            html! { <List /> }
        }
        _ => {
            html! {
                "Page not found"
            }
        }
    }
}
pub fn main() {
    yew::Renderer::<FullStackApp>::new().render();
}
