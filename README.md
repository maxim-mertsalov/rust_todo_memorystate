# Todo REST API working on memory state
This simple app with REST API

___
## REST API
The project has some routes:

Route | Method | Body | Description
`/` | `GET` | {} | Home page
`/todos` | `GET` | {} | Get all todos
`/todos` | `POST` | {title, date} | Create new todo
`/todos/{id}` | `PUT` | {title} | Update `id` todo
`/todos/{id}` | `DELETE` | {} | Delete `id` todo

## Dependencies
The project uses the following packages:
⋅⋅* [actix-web](https://github.com/actix/actix-web)
⋅⋅* [serde](https://github.com/serde-rs/serde)
⋅⋅* [serde_json](https://docs.rs/serde_json/latest/serde_json/)