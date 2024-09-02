use yew::{function_component, html, Html};

#[function_component]
pub fn RecipeTreeComponent() -> Html {
    html! {
        <ul class="tree">
            <li id="menu12">
                <label for="menu12">
                    <a>{"Recipes"}</a>
                </label>
                <input checked={false} id={"menu12"} value={""} type={"checkbox"} />
                <ul>
                    <li id="menu13">
                    <label for="menu13"><a>{"December"}</a></label>
                    <input checked={false} id={"menu13"} value={""} type={"checkbox"} />
                    <ul>
                        <li id="menu14">
                            <a>
                            <label for="menu14">{"Video"}</label>
                            <input checked={false} id="menu14" value="" type="checkbox" />
                            </a>
                        </li>
                        <li id="menu15">
                            <a>
                            <label for="menu14">{"Video"}</label>
                            <input checked={false} id="menu14" value="" type="checkbox" />
                            </a>
                        </li>
                    </ul>
                    </li>
                </ul>
            </li>
            <li id="menu15">
                <label for="menu15"><a>{"2014"}</a></label>
                <input checked={false} id="menu15" value="" type="checkbox" />
                <ul>

                    <li id="menu17">
                    <label for="menu17"><a>{"October"}</a></label>
                    <input checked={false} id="menu17" value="" type="checkbox" />
                    <ul>
                        <li id="menu18">
                            <a><label for="menu18">{"Video"}</label>
                            <input checked={false} id="menu18" value="" type="checkbox" />
                            </a>

                        </li>
                        <li id="menu18">
                            <a><label for="menu18">{"Video"}</label>
                            <input checked={false} id="menu18" value="" type="checkbox" />
                            </a>
                        </li>
                    </ul>
                    </li>
                </ul>
            </li>
            <li id="menu20">
            <a><label for="menu20">{"2013"}</label>
                <input checked={false} id={"menu20"} value={""} type={"checkbox"} />
            </a>
            </li>
        </ul>
    }
}
