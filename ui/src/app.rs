use ybc::NavbarFixed::Top;
use ybc::TileCtx::{Ancestor, Child, Parent};
use ybc::TileSize::Four;
use yew::prelude::*;

pub struct App; // An application component.

impl App {
    /* .. snip .. */
    pub fn view() -> Html {
        yew::html! {
          <>
          <ybc::Navbar fixed=Top /* .. your navbar content here .. *//>
          <ybc::Container fluid=true>
            <ybc::Tile ctx=Ancestor>
              <ybc::Tile ctx=Parent vertical=true size=Four>
                <ybc::Tile ctx=Child classes=classes!("box")>
                  <p>{"Lorem ipsum dolor sit amet ..."}</p>
                  <p>{"Lorem ipsum dolor sit amet ..."}</p>
                  <p>{"Lorem ipsum dolor sit amet ..."}</p>
                  <p>{"Lorem ipsum dolor sit amet ..."}</p>
                  <p>{"Lorem ipsum dolor sit amet ..."}</p>
                </ybc::Tile>
                /* .. snip .. more tiles here .. */
              </ybc::Tile>
            </ybc::Tile>
          </ybc::Container>
          </>
        }
    }
}
