---
title: "Basic Web App with Rust"
date: 2020-12-19T14:59:48+01:00
description: "This post describes the Rust stack for a basic web app that consists of a REST API, a Postgres database and a few views with JavaScript and CSS."
features:
    - Highlight
---

This post describes the Rust stack ([Actix Web](https://github.com/actix/actix-web), [Diesel](https://github.com/diesel-rs/diesel), [Askama](https://github.com/djc/askama)) for a basic web app that consists of a REST API, a Postgres database and a few views with JavaScript and CSS. The app is adding a backend to the editable graph from my [previous post](/posts/graph-editor-demo/). It allows creating and editing directed graphs. Each action is persisted in the database immediately. The code is on [GitHub](https://github.com/kldtz/graph-editor-app). Apart from the documentation and GitHub issues belonging to the different frameworks, I found Tore Pettersen's [tutorial on creating a REST API in Rust](https://cloudmaker.dev/how-to-create-a-rest-api-in-rust/) most helpful.


## Server and routes

The way you define methods and routes in *Actix Web* is very similar to the way it's done in Python frameworks like *FastAPI*. HTTP method and URL path are annotated on a Rust method. The request body, path and query parameters, as well as application state can be passed as arguments into the method. This is the hello-world example copied from the *Actix Web* documentation:

```rust
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```

An HTTP server is created from an application factory, which in this example is a closure that captures the index method defined in its environment. In the closure, an application instance is created, on which the index endpoint is registered. The factory is used to construct an application instance for each thread. It's possible to have shared data, but I don't need any in this simple app.


## Database

For interacting with the database, I use [**Diesel**](https://github.com/diesel-rs/diesel). It provides object-relational mapping (ORM) and versioned migrations. Each migration consists of two parts: one running the migration (e.g., creating a table), one reverting the migration (e.g., dropping the created table). In the simplest case, each part is an SQL file (`up.sql` and `down.sql`). If you don't like writing SQL, you can use [*barrel*](https://github.com/rust-db/barrel) with Diesel, which allows you to write your migrations in Rust. Diesel comes with its own CLI, that does the necessary setup, generates and runs/reverts migrations, similar to `rails db`. The interface is well-structured and documented. The command I found most useful during development was `diesel database reset`, which drops your database, recreates it and runs all migrations.

Diesel doesn't support asynchronous operations, so we need to offload our queries to a connection pool. I like the solution presented in [Tore Pettersen's blog post](https://cloudmaker.dev/how-to-create-a-rest-api-in-rust/) and blatantly copied the whole `db.rs` file from there. With this in place, you can write your models and queries. A model is a struct for which Diesel traits can be derived automatically. This is one of the models I use for the graph nodes:

```rust
#[derive(Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[belongs_to(parent = "Graph")]
#[table_name = "node"]
pub struct Node {
    pub id: i32,
    pub node_label: String,
    pub graph_id: i32,
    pub x_coord: f64,
    pub y_coord: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
```

The following traits are derived automatically:

* `Serialize/Deserialize`: generates (de)serialization implementations provided by [**Serde**](https://serde.rs/)
* `Queryable`: node struct can be the result of an SQL query
* `Identifiable`: node struct represents single DB row with an `id` method returning an identifier, prerequisite for Associations
* `Associations`: enables `belongs_to` relation to the parent model `Graph`

With this model, a simple query for a node with a given ID looks like this:

```rust
node::table.filter(node::id.eq(id)).first(&connection).unwrap()
```

The `filter` method adds the ID check to the `where` clause of the query, `first` attempts to load a single record from our pooled connection.

When creating a new node, I don't want to provide the ID and creation timestamp, since I set up Postgres to do this, so I have a second struct with a subset of fields, deriving the `Insertable` trait that provides us with an implementation of `insert_into`.

```rust
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "node"]
pub struct NodeInit {
    pub node_label: String,
    pub graph_id: i32,
    pub x_coord: f64,
    pub y_coord: f64,
}
```

To allow for flexible patches of subsets of attributes, you can define a model with optional fields like this:

```rust
#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "node"]
pub struct NodePatch {
    pub node_label: Option<String>,
    pub x_coord: Option<f64>,
    pub y_coord: Option<f64>,
}
```

The `AsChangeset` trait allows this struct to be passed to `update.set`. Optional fields will be skipped if their value is `None`.



## Template engine

To show some views to the user, it's nice to have a good template engine. I first tried the Rust version of [Handlebars](https://github.com/sunng87/handlebars-rust) because I was familiar with it from JavaScript, but later I switched to [**Askama**](https://github.com/djc/askama). The main difference between both is that *Askama* generates Rust code from the templates at compile time, taking full advantage of Rust's type system, while *Handlebars* processes the templates at runtime. Since all my templates are known at compile time, I don't need *Handlebar's* flexibility and can enjoy the advantages of *Askama*.

Here is an example of template inheritance. I defined a parent template with some minimal HTML structure around a content block.

```html
<!-- base.html -->
<!DOCTYPE html>
<html>

<head>
    <meta charset="UTF-8">
    <title>{{ title }}</title>
</head>

<body>
    <h1>{{title}}</h1>
    {% block content %}{% endblock %}
</body>

</html>
```

Another template extends this base template and defines the content block.

```html
{% extends "base.html" %}
{% block content %}

<div>
    <svg id="graph"></svg>
</div>

<script src="https://d3js.org/d3.v6.min.js"></script>
<script src="/static/graph.js"></script>

{% endblock content %}
```

In the code, define a struct for each template that contains the data used in the template. Derive *Askama's* `Template` trait and annotate the path to the template. All templates live in the `templates` directory under your project root. The path is relative to this directory.

```rust
#[derive(Template)]
#[template(path = "graphs/index.html")]
struct GraphsTemplate<'a> {
    title: &'a str,
    graphs: Vec<Graph>,
}
```

To compile the template, instantiate the struct with some data, for example with a page title and some objects from the database, and call the `render` method on it. The result can be used to create an HTTP response.

```rust
let graphs = Graph::find_all()?;
let template = GraphsTemplate {
    title: "Graphs",
    graphs: graphs,
};
let body = template.render()?;
HttpResponse::Ok().body(body)
```

## Static files

In the template above, I linked to a JavaScript file with a root-relative URL. To make that work, I serve a directory of static files by registering the static file handling service provided by `actix-files` on the app in the application factory.

```rust
use actix_files as fs;

App::new().service(fs::Files::new("/static", "./static"))
```