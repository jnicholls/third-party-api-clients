# `microsoft-graph-api`

A fully generated, opinionated API client library for Microsoft Graph API.

[![docs.rs](https://docs.rs/microsoft-graph-api/badge.svg)](https://docs.rs/microsoft-graph-api)

## API Details

This OData service is located at https://graph.microsoft.com/v1.0






## Client Details

This client is generated from the [Microsoft Graph API OpenAPI
specs](https://raw.githubusercontent.com/microsoftgraph/msgraph-metadata/master/openapi/v1.0/openapi.yaml) based on API spec version `v1.0`. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
microsoft-graph-api = "0.1.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use microsoft_graph_api::Client;

let microsoft_graph_api = Client::new(
    String::from("api-key"),
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `MICROSOFT_GRAPH_API_API_KEY`

And then you can create a client from the environment.

```
use microsoft_graph_api::Client;

let microsoft_graph_api = Client::new_from_env();
```
