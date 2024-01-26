use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Title text="Émile's Home"/>
        <div class="flex flex-col h-screen overflow-hidden gap-4 p-3">
            <Header/>
            <div class="flex-1 overflow-y-scroll">
                <Body/>
            </div>
            <Footer/>
        </div>
    }
}
/// Renders the header
#[component]
fn Header() -> impl IntoView {
    view! {
        <div class="flex justify-center">
            <div class="flex flex-row justify-center px-3 py-2 gap-3 border border-gray-200 rounded-lg shadow-md hover:shadow-lg">
                <h1 class="self-center text-3xl font-medium text-center">"Hi! I'm Émile"</h1>
                <img
                    class="size-12 hover:animate-wave_hand"
                    src="data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxZW0iIGhlaWdodD0iMWVtIiB2aWV3Qm94PSIwIDAgMTI4IDEyOCI+PHBhdGggZmlsbD0iI0ZGQ0EyOCIgZD0iTTE3LjU3IDYyLjY4Yy0yLjc5LTQuMDEtMy45MS03Ljc5LTEuMTgtMTAuMDhjMi4zMS0xLjk0IDUuNzEtMi4zMSA5LjkxIDIuNTRjMCAwIDEyLjU1IDE0LjU4IDE2Ljg3IDE4LjYzYy45OC45MSAyLjIgMS4wNSAzLjMxLS4xYy45NC0uOTguNi0xLjgzLS4yLTNjMCAwLTE3LjY5LTI2LjMtMjAuMDEtMjkuNTFjLTMuODctNS4zNy0yLjM4LTguODQtLjU5LTEwLjQ5YzIuNDktMi4zMSA2Ljg3LTIuNzcgMTAuOTQgMi44MWwyMS40MiAyOC42N2MuNjUuNjkgMS43Ljc5IDIuNDcuMjVsLjMtLjIxYy43OS0uNTYgMS4wMi0xLjYzLjU0LTIuNDdjLTMuNzUtNi41My0xOC42Ny0zMi41NS0yMC44Ni0zNy4yOWMtMi41Mi01LjQ3LTEuNDQtOC4yNSAxLjIzLTkuODZjMy4xNy0xLjkxIDYuMTUtMS43NyA5LjcxIDMuNTJjMy44NiA1Ljc2IDE4Ljg1IDMwLjAxIDIyLjY2IDM2LjUzYy41Ljg1IDEuNTcgMS4xNyAyLjQ1Ljc0Yy4wMS0uMDEuMDMtLjAxLjA0LS4wMmMuODQtLjQxIDEuNi0xLjI0IDEuMjUtMi40MmMtMi4wOC02Ljg2LTEyLjI5LTI4LjIyLTE0LjQzLTMzLjEzYy0yLjkzLTYuNzEtMS41LTguOTkgMS41My0xMC41M2MzLjE4LTEuNjEgNi40OS0uMzQgOC43NCA0LjE0YzEuNTIgMy4wNCAyOC4yMSA1MS42MSAyOC4yMSA1MS42MWMtLjM5LTcuMjQgMS40NC0xMi4zNyAzLTE3LjUyYzIuODQtOS4zNyA5LjcxLTE0LjI0IDE0LjY5LTEyLjA1YzIuNTkgMS4xNCAzLjA2IDMuNDEgMi43OCA1LjE1Yy0uNTYgMy4zOC0yLjk0IDEzLjg1LTMuNCAyMi4wNWMtMS4wOCAxOS4yMiA0LjczIDQxLjM3LTE2LjkyIDU1LjI5Yy0xNC40OSA5LjMyLTMwLjAyIDcuNjgtNDAuMjguNTFjLTEyLjQ0LTguNjktNDEuOS01MC40OC00NC4xOC01My43NiIvPjxwYXRoIGZpbGw9IiNFREE2MDAiIGQ9Ik0xMTcuNjggNTEuNzdjLTEuODEgNy42MS0yLjA1IDE2Ljk1LTEuOTkgMjAuOTNjLjI0IDE1LjUxLjE2IDI4LjkzLTE1LjM5IDQxLjE5Yy0xLjkxIDEuNTEtNy45IDUuMTktMTQuODcgNy4xMWMtMi4yNC42MS0xLjM5IDEuMzMtLjAxIDEuMTdjNy42MS0uODggMTMuMzgtNC4xNiAxNi41OS02LjIzYzIxLjY0LTEzLjkyIDE2LjMyLTM1LjkyIDE3LjQtNTUuMTRjLjQ2LTguMiAyLjk2LTIxLjEgMi45Mi0yMi4yMWMtLjA1LTEuMTEtMi44NCA1LjU3LTQuNjUgMTMuMThtLTQwLjE1LjhzLTEuNTUuMDEtMy4wMi0yLjA0QzcwLjEgNDQuNCA1Ni4yNyAyMS4yNSA1Mi4yOSAxNS41N2MtNC41NC02LjQ4LTguNzctNC05LjczLTMuNDhjMCAwIDMuNDguMTIgNS40IDIuOTdjMy45MiA1LjgxIDE4Ljc4IDMxLjg1IDIzLjQ3IDM3LjQ0YzMuMDMgMy42MSA2LjEuMDcgNi4xLjA3bS01NC44MiAxLjk3YzEuMzYgMS40NiAxMy4zIDE1LjYzIDE3LjcgMTkuNThjMy43OCAzLjM5IDYuNi0uOTMgNi42LS45M3MtMS4zOS4xOS0zLjI5LTEuMjRDMzkgNjguMzggMjcuODIgNTUuNjUgMjUuMyA1Mi45N2MtMy43NC0zLjk4LTcuMjItMS40OS03Ljk2LTEuMDRjLjAxIDAgMi4xNS0uODMgNS4zNyAyLjYxbTM4Ljg4IDYuNzJzLTEuMjguOTMtMy42My0xLjgyYy0xLjcxLTIuMDEtMjEuMzQtMjUuOTYtMjEuMzQtMjUuOTZjLTQuOTItNi4wMS04Ljc5LTQuMjItOS41OS0zLjc2YzAgMCAyLjQ3LS4xMyA1LjM2IDMuMmMxLjMyIDEuNTIgMjIuMjcgMjguMjMgMjMuMTEgMjkuMTRjMy4wNyAzLjM1IDUuOTEuNDcgNi4wOS0uOCIvPjxwYXRoIGZpbGw9IiNFREE2MDAiIGQ9Ik0xMDEuOCA1Ny44M1M3NS43OSAxNC4yMiA3NC4xMyAxMS4yNWMtMy42OS02LjU5LTcuOTgtNC41Ny04LjcxLTQuMjRjMCAwIDIuMzktLjY4IDUuMiA0LjFjMS41IDIuNTQgMjAuNjEgMzYuOSAyNi43MSA0Ny44OGMuNDggMi4wNS40NCA0LjcyLTEuMSA2LjQ0Yy01LjUyIDYuMTktMTIuNTYgMTIuNTEtMTAuNDkgMjguMThjLjUyIDMuOTcgMS45OSA3LjczIDMuMDggOS41NGMxLjM4IDIuMjcgMi45NyAxLjQxIDIuMjctLjI5Yy0uNDctMS4xNS0xLjIyLTMuODYtMS4zNy01LjAyYy0uNy01LjQtMy4wNi0xNC44NCA3LjUyLTI2LjE2YzEuNzktMS44OSA2LjM0LTcuNiA0LjU2LTEzLjg1Ii8+PHBhdGggZmlsbD0iI0IwQkVDNSIgZD0iTTEwMy40OSAzMC45NmMtMS4zOS00LjkzLTMuNTUtOS40NS02LjM1LTEzLjM3Yy0yLjUyLTMuNTMtNS41Ny02LjU3LTkuMDUtOS4wMWMtLjQ0LS4zMS0xLTEuMjctLjUyLTIuMnMxLjUyLS44MSAxLjkxLS42NWM0LjM1IDEuNzkgNy45MSA0Ljg4IDEwLjg1IDkuMTJjMy40OSA1LjA0IDUuMjIgOS43OSA2LjExIDE1LjFjLjEuNTguMDggMS44LTEuMTYgMi4xMmMtMS4yMy4zMi0xLjY1LS42My0xLjc5LTEuMTFtLTYuOCA1LjI5Yy0xLjM5LTQuOTMtMy41NS05LjQ1LTYuMzUtMTMuMzdjLTIuNTItMy41My01LjU3LTYuNTctOS4wNS05LjAxYy0uNDQtLjMxLTEtMS4yNy0uNTItMi4yYy40OC0uOTQgMS41Mi0uODEgMS45MS0uNjVjNC4zNSAxLjc5IDcuOTEgNC44OCAxMC44NSA5LjEyYzMuNDkgNS4wNCA1LjIyIDkuNzkgNi4xMSAxNS4xYy4xLjU4LjA4IDEuOC0xLjE2IDIuMTJjLTEuMjIuMzItMS42NS0uNjItMS43OS0xLjExTTguNTYgNzcuNDZjMS43MiA0LjgzIDQuMTcgOS4xOSA3LjIyIDEyLjkyYzIuNzUgMy4zNSA1Ljk5IDYuMTkgOS42MiA4LjRjLjQ2LjI4IDEuMDggMS4yLjY2IDIuMTZjLS40Mi45Ny0xLjQ2LjkxLTEuODcuNzhjLTQuNDYtMS41LTguMjItNC4zNS0xMS40My04LjM5Yy0zLjgxLTQuOC01Ljg2LTkuNDItNy4xLTE0LjY2Yy0uMTQtLjU4LS4yLTEuNzkgMS4wMi0yLjE5YzEuMjItLjQgMS43MS41MSAxLjg4Ljk4bTYuNDMtNS43M2MxLjcyIDQuODMgNC4xNyA5LjE5IDcuMjIgMTIuOTJjMi43NSAzLjM1IDUuOTkgNi4xOSA5LjYyIDguNGMuNDYuMjggMS4wOCAxLjIuNjYgMi4xNmMtLjQyLjk3LTEuNDYuOTEtMS44Ny43OGMtNC40Ni0xLjUtOC4yMi00LjM1LTExLjQzLTguMzljLTMuODEtNC44LTUuODYtOS40Mi03LjEtMTQuNjZjLS4xNC0uNTgtLjItMS43OSAxLjAyLTIuMTljMS4yMi0uNDEgMS43MS41MSAxLjg4Ljk4Ii8+PC9zdmc+"
                    alt=""
                />
            </div>
        </div>
    }
}

/// Page Body
#[component]
fn Body() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the header
#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="sticky">
            <div class="flex justify-center">
                <div class="flex flex-row justify-center px-3 py-2 gap-3 border border-gray-200 rounded-lg shadow-md hover:shadow-lg">
                    <a href="https://github.com/EmileVezinaCoulombe/" target="_blank">
                        <span class="icon-[mdi--github] size-[40px]"></span>
                    </a>
                    <a href="https://www.linkedin.com/in/%C3%A9mile-v%C3%A9zina-coulombe-2b9534207/" target="_blank">
                        <span class="icon-[mdi--linkedin] size-[40px]"></span>
                    </a>
                    <a href="mailto:emilevezinacoulombe@icloud.com">
                        <span class="icon-[material-symbols--mail] size-[40px]"></span>

                    </a>
                    <a href="https://www.google.com/maps/place/Lévis,+QC/" target="_blank">
                        <span class="icon-[uiw--map] size-[40px]"></span>
                    </a>
                    <a href="https://github.com/EmileVezinaCoulombe/emilevezinacoulombe/assets/CV_Émile_Vézina-Coulombe_2024.pdf" target="_blank">
                        <span class="icon-[pepicons-pop--cv] size-[40px]"></span>
                    </a>
                </div>
            </div>
        </footer>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col justify-center">
            <h3 class="text-xl font-medium text-center">"Under active construction"</h3>
            <div class="flex justify-center m-4">
                <img
                    class="size-24 hover:animate-shake_fast animate-spin"
                    src="data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxZW0iIGhlaWdodD0iMWVtIiB2aWV3Qm94PSIwIDAgMTI4IDEyOCI+PHBhdGggZmlsbD0iIzlFOUU5RSIgZD0iTTI4LjE5IDEyMS43N3MtMS43NSAyLjIzLTYuMyAyLjIzcy02LjMtMi4yMy02LjMtMi4yM1Y0My43MWgxMi42eiIvPjxwYXRoIGZpbGw9IiNCREJEQkQiIGQ9Ik0yOC4xOSAxMjEuNzdzLTEuNzUgMi4yMy02LjMgMi4yM3MtNi4zLTIuMjMtNi4zLTIuMjNWOTEuN3MxLjI4LTIuNDQgNS45Ni0yLjQ0czYuNjQgMi40NCA2LjY0IDIuNDR6Ii8+PHBhdGggZmlsbD0iIzlFOUU5RSIgZD0iTTExMi40IDEyMS43N3MtMS43NSAyLjIzLTYuMyAyLjIzcy02LjMtMi4yMy02LjMtMi4yM1Y0My43MWgxMi42eiIvPjxwYXRoIGZpbGw9IiNCREJEQkQiIGQ9Ik0xMTIuNCAxMjEuNzdzLTEuNzUgMi4yMy02LjMgMi4yM3MtNi4zLTIuMjMtNi4zLTIuMjNWOTEuN3MxLjI4LTIuNDQgNS45Ni0yLjQ0czYuNjQgMi40NCA2LjY0IDIuNDR6Ii8+PHBhdGggZmlsbD0iIzlFOUU5RSIgZD0iTTEyMS4yNyA4MS44Nkg2LjczQzUuMjIgODEuODYgNCA4MC42NCA0IDc5LjEzVjQ0LjE3YzAtMS41MSAxLjIyLTIuNzMgMi43My0yLjczaDExNC41NGMxLjUxIDAgMi43MyAxLjIyIDIuNzMgMi43M3YzNC45NmMwIDEuNTEtMS4yMiAyLjczLTIuNzMgMi43MyIvPjxwYXRoIGZpbGw9IiNGRkQ2MDAiIGQ9Ik0xMjEuMjcgNDMuNDRINi43M0M1LjIyIDQzLjQ0IDQgNDQuNjYgNCA0Ni4xN3YzNC45NmMwIDEuNTEgMS4yMiAyLjczIDIuNzMgMi43M2gxMTQuNTRjMS41MSAwIDIuNzMtMS4yMiAyLjczLTIuNzNWNDYuMTdjMC0xLjUxLTEuMjItMi43My0yLjczLTIuNzMiLz48cGF0aCBmaWxsPSIjMjEyMTIxIiBkPSJNMTcuNDMgNDMuNDRMNCA1Ni44N3YxOS44bDMzLjIzLTMzLjIzem0zNy42NyAwTDE0LjY5IDgzLjg2aDE5LjhMNzQuOSA0My40NHptMzcuNjggMEw1Mi4zNiA4My44NmgxOS44bDQwLjQxLTQwLjQyek0xMjQgNDkuODlMOTAuMDMgODMuODZoMTkuOEwxMjQgNjkuNjl6Ii8+PHBhdGggZmlsbD0iI0UyQTYxMCIgZD0iTTExMy44IDQ1LjE5SDk4LjQxYy0xLjQ0IDAtMi41LS45NC0yLjE2LTEuOWwxLjA3LTYuNzFoMTcuNTVsMS4wNyA2LjcxYy4zNi45Ny0uNyAxLjktMi4xNCAxLjkiLz48Y2lyY2xlIGN4PSIxMDYuMSIgY3k9IjIzLjcxIiByPSIxMi43MSIgZmlsbD0iI0ZGQ0EyOCIvPjxjaXJjbGUgY3g9IjEwNi4xIiBjeT0iMjMuNzEiIHI9IjkuMzkiIGZpbGw9IiNGRjU3MjIiLz48cGF0aCBmaWxsPSIjRkZDQTI4IiBkPSJNMTE0Ljg4IDM2LjY4SDk3LjMzcy4wNi0yLjggNS4wOC0yLjhoNy40NGM1LjMxIDAgNS4wMyAyLjggNS4wMyAyLjgiLz48cGF0aCBmaWxsPSIjRkZENUNBIiBkPSJtMTA3LjIxIDE5LjIxbDIuNC0zLjFjLjExLS4xNC4zMy0uMDMuMjkuMTRsLS45MyAzLjgxYy0uMjIuODkuNDYgMS43NSAxLjM4IDEuNzNsMy45Mi0uMDZjLjE4IDAgLjIzLjI0LjA3LjMybC0zLjU2IDEuNjVjLS44My4zOS0xLjA3IDEuNDUtLjQ5IDIuMTZsMi40OSAzLjAzYy4xMS4xNC0uMDQuMzMtLjIuMjVsLTMuNTEtMS43NWMtLjgyLS40MS0xLjguMDYtMiAuOTZsLS44MiAzLjg0Yy0uMDQuMTgtLjI5LjE4LS4zMiAwbC0uODItMy44NGMtLjE5LS45LTEuMTgtMS4zNy0yLS45NmwtMy41MSAxLjc1Yy0uMTYuMDgtLjMyLS4xMi0uMi0uMjVsMi40OS0zLjAzYy41OC0uNzEuMzQtMS43Ny0uNDktMi4xNmwtMy41Ni0xLjY1Yy0uMTYtLjA4LS4xMS0uMzIuMDctLjMybDMuOTIuMDZjLjkyLjAxIDEuNi0uODQgMS4zOC0xLjczbC0uOTMtMy44MWMtLjA0LS4xNy4xOC0uMjguMjktLjE0bDIuNCAzLjFjLjU5LjcyIDEuNjguNzIgMi4yNCAwIi8+PHBhdGggZmlsbD0iI0UyQTYxMCIgZD0iTTI5LjU5IDQ1LjE5SDE0LjJjLTEuNDQgMC0yLjUtLjk0LTIuMTYtMS45bDEuMDctNi43MWgxNy41NWwxLjA3IDYuNzFjLjM2Ljk3LS43IDEuOS0yLjE0IDEuOSIvPjxjaXJjbGUgY3g9IjIxLjkiIGN5PSIyMy43MSIgcj0iMTIuNzEiIGZpbGw9IiNGRkNBMjgiLz48Y2lyY2xlIGN4PSIyMS45IiBjeT0iMjMuNzEiIHI9IjkuMzkiIGZpbGw9IiNGRjU3MjIiLz48cGF0aCBmaWxsPSIjRkZDQTI4IiBkPSJNMzAuNjcgMzYuNjhIMTMuMTJzLjA2LTIuOCA1LjA4LTIuOGg3LjQ0YzUuMzEgMCA1LjAzIDIuOCA1LjAzIDIuOCIvPjxwYXRoIGZpbGw9IiNGRkQ1Q0EiIGQ9Im0yMyAxOS4yMWwyLjQtMy4xYy4xMS0uMTQuMzMtLjAzLjI5LjE0bC0uOTMgMy44MWMtLjIyLjg5LjQ2IDEuNzUgMS4zOCAxLjczbDMuOTItLjA2Yy4xOCAwIC4yMy4yNC4wNy4zMmwtMy41NiAxLjY1Yy0uODMuMzktMS4wNyAxLjQ1LS40OSAyLjE2bDIuNDkgMy4wM2MuMTEuMTQtLjA0LjMzLS4yLjI1bC0zLjUxLTEuNzVjLS44Mi0uNDEtMS44LjA2LTIgLjk2bC0uODIgMy44NGMtLjA0LjE4LS4yOS4xOC0uMzIgMGwtLjgyLTMuODRjLS4xOS0uOS0xLjE4LTEuMzctMi0uOTZsLTMuNTEgMS43NWMtLjE2LjA4LS4zMi0uMTItLjItLjI1bDIuNDktMy4wM2MuNTgtLjcxLjM0LTEuNzctLjQ5LTIuMTZsLTMuNTYtMS42NWMtLjE2LS4wOC0uMTEtLjMyLjA3LS4zMmwzLjkyLjA2Yy45Mi4wMSAxLjYtLjg0IDEuMzgtMS43M2wtLjkzLTMuODFjLS4wNC0uMTcuMTgtLjI4LjI5LS4xNGwyLjQgMy4xYy41OS43MiAxLjY4LjcyIDIuMjQgMCIvPjwvc3ZnPg=="
                    alt=""
                />
            </div>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    view! {
        <h1>"Not Found"</h1>
        <a href="/">"Going Home?"</a>
    }
}

