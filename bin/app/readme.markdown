# mbp2 Backend-Database Srv

This package contains the application backend.

### Database
A connection to a remote Atlas instance is made, and queries are made via the `mongodb` package.
Our database instance is passed throughout the program via cloning since it is a relatively low-cost
operation to perform, therefore we only need to make a connexion once to the database. Our config
has been set as such (within the `config.json` at the package root):
- Write Concern: "majority"
- Retry Reads: true
- Retry Writes: true
- Local Threshold: 100ms

### Server
The `rocket` package is used to provide a web server. This is configured via the `Rocket.toml` and
the `config.json` in the project root. Meaningful config options include:
- Address binding: "localhost:8000" (binds our webserver to the local machine, port 8000);
- Max socket payload size: deliberately left unset for the time being;
- Socket host override: deliberately set to null for the time being;
- Client settings include:
   - Author client vars, including a username, keywords, and a summary. (These options are here for
     when federation is set up);
   - Custom CSS in the `client.customStyles` key;
   - Client-side notifications, to be set via cookie.
- Database options (as seen above);
- Auth0 options for remote key and credential authentication.

The `config.json` will be automatically generated upon first run, and will initially result in a
runtime failure. Issues may be remedied by setting necessary configuration options in the `config.json`.
