use yew::prelude::*;

#[function_component]
pub fn Login() -> Html {
    html! {
        <>
            <div class={classes! {"header"}}>
              <h1>{"Zulu"}</h1>
              
            </div>

            <div class={classes! {"topnav"}}>
              <a href="#">{"Home"}</a>
              <a href="#" style="float:right">{"Login"}</a>
            </div>
            <div class={classes! {"box"}}>
                <h2 style="color:chartreuse">{"Login"}</h2>
                <form method="post" action="http://127.0.0.1:8000/api/v1/login">
                    <label for="studentid">{"Username"}</label>
                    <input type="text" id="studentid" name="studentid" placeholder="Your Student ID.." style="color:black;" />
                    <br/>
                    <label for="Password">{"Password"}</label>
                    <input type="text" id="Password" name="password" placeholder="Your Password.." style="color:black;" />
                    <br/>
                    <input id="submit" type="submit" value="Submit" />
                  </form>

            </div>
        </>
    }
}
