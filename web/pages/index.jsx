let AUTH0_CLIENT_ID = "";
let AUTH0_DOMAIN = "";
const AUTH0_CALLBACK_URL = location.href;
const AUTH0_API_AUDIENCE = "https://api.otherskies.blog";

class App extends React.Component {
   parseHash() {
      this.auth0 = new auth0.WebAuth({
         domain: AUTH0_DOMAIN,
         clientID: AUTH0_CLIENT_ID,
      });

      this.auth0.parseHash(window.location.hash, (err, authResult) => {
         if (err) {
            return console.log(err);
         }

         if (
             authResult !== null &&
             authResult.accessToken !== null &&
             authResult.idToken !== null
         ) {
            localStorage.setItem("access_token", authResult.accessToken);
            localStorage.setItem("id_token", authResult.idToken);
            localStorage.setItem("profile", JSON.stringify(authResult.idTokenPayload));

            window.location = window
                .location
                .href
                .substring(0, window.location.href.indexOf("#"));
         }
      });
   }

   setup() {
      $.ajaxSetup({
         beforeSend: (r) => {
            if (localStorage.getItem("access_token")) {
               r.setRequestHeader(
                   "Authorization",
                   "Bearer " + localStorage.getItem("access_token")
               )
            }
         }
      })
   }

   setState() {
      let idToken = localStorage.getItem("id_token");
      this.loggedIn = !!idToken;
   }

   componentDidMount() {
      this.setup();
      this.parseHash();
      this.setState();
   }

   render() {
      if (this.loggedIn) {
         return (<LoggedIn />);
      } else {
         return (<Home />);
      }
   }
}

class Home extends React.Component {
   render() {
      return (
         <div className="container">
            <div className="col-xs-8 col-xs-offset-2 jumbotron text-center">
               <h1>Other Skies Blog</h1>
               <div className={"container row"}>
                  <span>
                     <a rel="me" href="https://social.undrground.org/@az">@az@social.undrground.org</a>
                  </span>
               </div>
               <p>Please sign in for access to donation perks.</p>
               <a onClick={this.authenticate} className={"btn btn-primary btn-lg btn-login btn-block"}>Sign In</a>
               <p>
                  Please consider becoming a Patron today if you are not already!
               </p>
               <iframe id='kofiframe'
                       src='https://ko-fi.com/azyklus/?hidefeed=true&widget=true&embed=true&preview=true'
                       style={{border: 8 + 'px', width:100 + '%', padding:4 + 'px', background:'#f9f9f9'}}
                       height='720'
                       title='azyklus'>
               </iframe>
            </div>
         </div>
      );
   }
}

class LoggedIn extends React.Component {
   constructor(props) {
      super(props);
      this.state = {
         entries: []
      }
   }

   render() {
      return (
         <div className="container">
            <div className="col-xs-8 col-xs-offset-2 jumbotron text-center">
               <h1>Other Skies Blog</h1>
               <div className={"container row"}>
                  <span>
                     <a rel="me" href="https://social.undrground.org/@az">@az@social.undrground.org</a>
                  </span>
               </div>
               <p>
                  Please consider becoming a Patron if you are not already!
               </p>
               <iframe id='kofiframe'
                       src='https://ko-fi.com/azyklus/?hidefeed=true&widget=true&embed=true&preview=true'
                       style={{border: 8 + 'px', width:100 + '%', padding:4 + 'px', background:'#f9f9f9'}}
                       height='720'
                       title='azyklus'>
               </iframe>
            </div>

            <div className="col-lg-12">
               <br />
               <span className="text-center">
                  <a onClick={this.logout} className="btn btn-primary btn-lg btn-logout btn-block">Log out</a>
               </span>

               <p>A feed of our latest entries:</p>
               <div className="col">
                  {
                     this.state.entries.map(function(entry, index) {
                        return (
                           <BlogPostPreview key={index} entry={entry} />
                        );
                     })
                  }
               </div>
            </div>
         </div>
      );
   }
}

class BlogPost extends React.Component {
   render() {
      return (
         <div></div>
      );
   }
}

class BlogPostPreview extends React.Component {
   render() {
      return (
         <div></div>
      );
   }
}

ReactDOM.render(<App />, document.getElementById("app"))
