pub struct Navbar {
   navActive: bool,
}

#[derive(Debug, PartialEq, Properties)]
pub struct NavbarProps {
   pub title: String,
}

impl Component for Navbar {
   type Message = Msg;
   type Properties = NavbarProps;

   fn create(_: &Context<Self>) -> Self {
      return Navbar{
         navActive: false,
      };
   }

   fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
      return match msg {
         Msg::ToggleNav => {
            self.navActive = !self.navActive;
            true
         }
      };
   }

   fn view(&self, ctx: &Context<Self>) -> Html {
      let Self { navActive, .. } = *self;
      let NavbarProps { title } = ctx.props();
      let activeClass = if !navActive { "is-active" } else { "" };

      let socialsLink = html!(
         <Container>
            { "Socials" }
         </Container>
      );

      let codeLink = html!(
         <Container>
            { "Code" }
         </Container>
      );

      return html!(
         <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
               <img class="navbar-item is-size-1" type="image/svg+xml" src="/static/logo.svg" width="60px" height="60px" />
               <h1 class="navbar-item is-size-3">{ title }</h1>

               <button class={classes!("navbar-burger", "burger", activeClass)}
                  aria-label="menu" aria-expanded="false"
                  onclick={ctx.link().callback(|_| Msg::ToggleNav)}
               >
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
               </button>
            </div>
            <div class={classes!("navbar-menu", activeClass)}>
               <div class="navbar-start">
                  <Link<Route> classes={classes!("navbar-item", "no-decoration")} to={Route::Home}>
                     { "Home" }
                  </Link<Route>>
                  <Link<Route> classes={classes!("navbar-item", "no-decoration")} to={Route::PostIndex}>
                     { "Posts" }
                  </Link<Route>>
                  <Link<Route> classes={classes!("navbar-item", "no-decoration")} to={Route::AuthorIndex}>
                     { "Authors" }
                  </Link<Route>>
                  <NavbarDropdown navlink={socialsLink} hoverable=true>
                     <a class="navbar-item" href="https://instagram.com/azyklus">{ "Instagram" }</a>
                     <a class="navbar-item" href="https://elk.zone/mas.to/@zub">{ "Mastodon" }</a>
                     <a class="navbar-item" href="https://twitch.tv/azyklus">{ "Twitch" }</a>
                     <a class="navbar-item" href="https://twitter.com/azyklus">{ "Twitter" }</a>
                     <a class="navbar-item" href="https://youtube.com/@azyklus">{ "YouTube" }</a>
                  </NavbarDropdown>
                  <NavbarDropdown navlink={codeLink} hoverable=true>
                     <a class="navbar-item" href="https://github.com/azyklus">{ "GitHub" }</a>
                     <a class="navbar-item" href="https://sr.ht/~azyklus">{ "Sourcehut" }</a>
                  </NavbarDropdown>
               </div>
            </div>
         </nav>
      );
   }
}

use {
   crate::{Msg, Route},
   router::prelude::*,
   ybc::*,
   yew::prelude::*
};
