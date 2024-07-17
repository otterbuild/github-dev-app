# `github-dev-app`

`github-dev-app` is a command-line application that helps set up local
development environments for GitHub Apps. It can create a new GitHub App from a
manifest and write the app's secrets to the local `.env` file.

## Installation

`github-dev-app` can be installed using the following command:

```shell
cargo install --git https://github.com/otterbuild/github-dev-app
```

## Usage

The CLI and its command can be browsed by calling the help command:

```shell
github-dev-app --help
```

### Register a new GitHub App

To register a new GitHub App, you need to provide a manifest file. The manifest
file is a JSON file that contains the app's metadata. The schema of the file is
documented here:
<https://docs.github.com/en/apps/sharing-github-apps/registering-a-github-app-from-a-manifest#github-app-manifest-parameters>.

The registration process can be started by running the following command:

```shell
github-dev-app register --manifest <path-to-manifest>
```

This opens a browser window where you can log in to GitHub and authorize the
app registration. After the registration is complete, the app's secrets are
written to the local `.env` file.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT)
  or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
