use leptos::prelude::*;
use leptos::mount::mount_to_body;
use leptos_router::{components::*, path};

mod moviespage;
use crate::moviespage::MovCatListPage;

mod tvshowspage;
use crate::tvshowspage::TVShowsListPage;

mod searchpage;
use crate::searchpage::SearchPage;

mod movpages {
    pub mod movapages;
    pub mod movbpages;
    pub mod movcpages;
    pub mod movdpages;
    pub mod movfpages;
    pub mod movgpages;
    pub mod movhpages;
    pub mod movipages;
    pub mod movjpages;
    pub mod movkpages;
    pub mod movlpages;
    pub mod movmpages;
    pub mod movnpages;
    pub mod movopages;
    pub mod movppages;
    pub mod movrpages;
    pub mod movspages;
    pub mod movtpages;
    pub mod movvpages;
    pub mod movxpages;
}
use crate::movpages::movapages::{ActionPage, ArnoldPage};
use crate::movpages::movbpages::{BruceLeePage, BruceWillisPage, BuzzPage};
use crate::movpages::movcpages::{CartoonsPage, CharlieBrownPage, ChuckNorrisPage, ComedyPage};
use crate::movpages::movdpages::{DramaPage, DocumentaryPage};
use crate::movpages::movfpages::FantasyPage;
use crate::movpages::movgpages::{GhostBustersPage, GodzillaPage};
use crate::movpages::movhpages::{HarrisonFordPage, HarryPotterPage, HellBoyPage};
use crate::movpages::movipages::IndianaJonesPage;
use crate::movpages::movjpages::{JamesBondPage, JohnWaynePage, JohnWickPage, JurassicParkPage};
use crate::movpages::movkpages::KevinCostnerPage;
use crate::movpages::movkpages::KingsManPage;
use crate::movpages::movlpages::LegoPage;
use crate::movpages::movmpages::{MenInBlackPage, MinionsPage, MiscPage};
use crate::movpages::movnpages::NicolasCagePage;
use crate::movpages::movopages::OldiesPage;
use crate::movpages::movppages::PandasPage;
use crate::movpages::movppages::PiratesPage;
use crate::movpages::movrpages::RiddickPage;
use crate::movpages::movspages::{SciFiPage, StalonePage, StarTrekPage, StarWarsPage, SuperHerosPage};
use crate::movpages::movtpages::{TinkerBellPage, TomCruisePage, TransformersPage, TremorsPage, TheRockPage};
use crate::movpages::movvpages::VanDamPage;
use crate::movpages::movxpages::XMenPage;

mod tvpages {
    pub mod tvactionpage;
    pub mod tvcomedypage;
    pub mod tvfantasypage;
    pub mod tvmcupage;
    pub mod tvsciencepage;
    pub mod tvscifipage;
    pub mod tvstartrekpage;
    pub mod tvstarwarspage;
    pub mod tvwesternspage;
}

use crate::tvpages::tvactionpage::TVActionPage;
use crate::tvpages::tvcomedypage::TVComedyPage;
use crate::tvpages::tvfantasypage::TVFantasyPage;
use crate::tvpages::tvmcupage::TVMCUPage;
use crate::tvpages::tvsciencepage::TVSciencePage;
use crate::tvpages::tvscifipage::TVSciFiPage;
use crate::tvpages::tvstartrekpage::TVStarTrekPage;
use crate::tvpages::tvstarwarspage::TVStarWarsPage;
use crate::tvpages::tvwesternspage::TVWesternsPage;

mod seasonpages {
    pub mod tvcontinentalseapage;
    pub mod tvfobarseapage;
    pub mod tvshogunseapage;
    
    
}

use crate::seasonpages::tvcontinentalseapage::TVContinentalSeaPage;
use crate::seasonpages::tvfobarseapage::TVFobarSeaPage;
use crate::seasonpages::tvshogunseapage::TVShogunSeaPage;


fn main() {
	console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <NavBar />
            <Header />
            <main>
                <Routes fallback=|| "Not Found.">
                    <Route path=path!("/") view=MovCatListPage />
                    <Route path=path!("/tvshows") view=TVShowsListPage />
                    <Route path=path!("/search") view=SearchPage />
                    <Route path=path!("/action") view=ActionPage />
                    <Route path=path!("/arnold") view=ArnoldPage />
                    <Route path=path!("/brucelee") view=BruceLeePage />
                    <Route path=path!("/brucewillis") view=BruceWillisPage />
                    <Route path=path!("/buzz") view=BuzzPage />
                    <Route path=path!("/cartoons") view=CartoonsPage />
                    <Route path=path!("/charliebrown") view=CharlieBrownPage />
                    <Route path=path!("/chucknorris") view=ChuckNorrisPage />
                    <Route path=path!("/comedy") view=ComedyPage />
                    <Route path=path!("/documentary") view=DocumentaryPage />
                    <Route path=path!("/drama") view=DramaPage />
                    <Route path=path!("/fantasy") view=FantasyPage />
                    <Route path=path!("/ghostbuster") view=GhostBustersPage />
                    <Route path=path!("/godzilla") view=GodzillaPage />
                    <Route path=path!("/harrisonford") view=HarrisonFordPage />
                    <Route path=path!("/harrypotter") view=HarryPotterPage />
                    <Route path=path!("/hellboy") view=HellBoyPage />
                    <Route path=path!("/indianajones") view=IndianaJonesPage />
                    <Route path=path!("/jamesbond") view=JamesBondPage />
                    <Route path=path!("/johnwayne") view=JohnWaynePage />
                    <Route path=path!("/johnwick") view=JohnWickPage />
                    <Route path=path!("/jurassicpark") view=JurassicParkPage />
                    <Route path=path!("/kevincostner") view=KevinCostnerPage />
                    <Route path=path!("/kingsman") view=KingsManPage />
                    <Route path=path!("/lego") view=LegoPage />
                    <Route path=path!("/meninblack") view=MenInBlackPage />
                    <Route path=path!("/minions") view=MinionsPage />
                    <Route path=path!("/misc") view=MiscPage />
                    <Route path=path!("/nicolascage") view=NicolasCagePage />
                    <Route path=path!("/oldies") view=OldiesPage />
                    <Route path=path!("/pandas") view=PandasPage />
                    <Route path=path!("/pirates") view=PiratesPage />
                    <Route path=path!("/riddick") view=RiddickPage />
                    <Route path=path!("/scifi") view=SciFiPage />
                    <Route path=path!("/stalone") view=StalonePage />
                    <Route path=path!("/startrek") view=StarTrekPage />
                    <Route path=path!("/starwars") view=StarWarsPage />
                    <Route path=path!("/superheros") view=SuperHerosPage />
                    <Route path=path!("/tinkerbell") view=TinkerBellPage />
                    <Route path=path!("/tomcruise") view=TomCruisePage />
                    <Route path=path!("/transformers") view=TransformersPage />
                    <Route path=path!("/tremors") view=TremorsPage />
                    <Route path=path!("/therock") view=TheRockPage />
                    <Route path=path!("/vandam") view=VanDamPage />
                    <Route path=path!("/xmen") view=XMenPage />

                    <Route path=path!("/tvactionpage") view=TVActionPage />
                    <Route path=path!("/tvcomedypage") view=TVComedyPage />
                    <Route path=path!("/tvfantasypage") view=TVFantasyPage />
                    <Route path=path!("/tvmcupage") view=TVMCUPage />
                    <Route path=path!("/tvscience") view=TVSciencePage />
                    <Route path=path!("/tvscifi") view=TVSciFiPage />
                    <Route path=path!("/tvstartrek") view=TVStarTrekPage />
                    <Route path=path!("/tvstarwars") view=TVStarWarsPage />
                    <Route path=path!("/tvwesterns") view=TVWesternsPage />

                    <Route path=path!("/tvcontinentalseapage") view=TVContinentalSeaPage />
                    <Route path=path!("/tvfobarseapage") view=TVFobarSeaPage />
                    <Route path=path!("/tvshogunseapage") view=TVShogunSeaPage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header>
            <h1 class="header">"MTV"</h1>
        </header>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <nav>
            <a href="/" class="navItem">"Movies"</a>
            <a href="/tvshows" class="navItem">"TV Shows"</a>
            <a href="/search" class="navItem">"Search"</a>
        </nav>
    }
}