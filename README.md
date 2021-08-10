AssemblyLift Jamstack Template
------------------------------

The repository provides a template for bootstrapping an AssemblyLift [Jamstack](https://jamstack.org) application.

[AssemblyLift](https://assemblylift.akkoro.io) **v0.3.1** or greater is required.
Install assemblylift with `cargo install assemblylift-cli`.

The `web` directory is a simple site built by webpack to demonstrate how a frontend can be integrated with an AssemblyLift project.
This could be replaced by full-featured framework such as React if desired.

### Building
- First run `npm install` to install webpack et al.
- The web frontend is built by running `npm run build`. The output of webpack is `./dist`, the contents of which will
be embedded in your deployed function.
- The AssemblyLift services can be built as usual with `asml cast` and deployed with `asml bind`.
  - ⚠️ You will need an AWS account & credentials

See the [AssemblyLift documentation](https://docs.assemblylift.akkoro.io) for more details.
  
### How it works
The function `www/server` uses the `rust-embed` crate to embed the contents of the webpack `dist` directory in the compiled 
WebAssembly binary. When invoked, the function proxies the path and attempts to match & return one of the embedded assets, 
returning a 404 if an exact match is not found.
