# Aletheia

Aletheia is an API for as many services as possible. Right now, it's
just going to power HackNYU's site, but perhaps it will be used for
other services.

## Technical Overview

Aletheia uses Rocket for the server, Diesel as the ORM, and Juniper
for GraphQL. There is both a REST API and a GraphQL API. The database
is PostgreSQL.

## Folders

**db/models** --- Database models and related structs. There's also a
few helper methods.

**routes** --- The routes for the REST API. Consists of routes by
resource (i.e. project, media, etc). Generally should just process the
request and route to the corresponding resolver.

**graphql** --- The folder for GraphQL types. Consists of files for
each type, declaring the fields and linking them to resolvers. The
GraphQL equivalent of **routes**

**resolvers** --- Kinda like controllers from MVC. Fetches data from
the database, calls the service functions and generally links the
interfaces (**routes**, **graphql**) to the models.

**services** --- For any logic that is not data fetching. Service
functions are files with one main function called `call` in it. They
are nested according to their usage. For instance, for image
processing, we have a resizing service function, `image/resize.rs` and
an uploading service function `image/upload.rs`. Other code like
general validation (NOT token or permission validation) should go here.

**types** --- Misc types that are not stored in the database.

**migrations** --- Database migrations. Whenever we need to change the
schema, we make a migration and run it. This allows us to transitions
schema while keeping our data intact. Do NOT delete migrations or edit
existing ones.

## Misc Files

**utils** --- Miscellaneous stuff for now. Also a Result type that we
should generally use (`Result<T, failure::Error`)

## Contributing

We welcome contributors! Anybody is welcome to open a pull
request. Please run [rustfmt](https://github.com/rust-lang/rustfmt) to
format the code, and ensure that the code works on nightly. If you
need some guidance on how to contribute, feel free to open an
issue. We can point you to some issues and resources.
