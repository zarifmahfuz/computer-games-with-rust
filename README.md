# computer-games-with-rust

## Installation
Please follow this link https://yew.rs/docs/getting-started/introduction to install the dependencies for this project.

## Run

Move to the `backend` directory and follow the instructions provided over there. Then, run the backend server with `cargo run`.

Open another terminal and move into the `front_end` directory. Then, run the web application with:

```
trunk serve --proxy-backend=http://127.0.0.1:5000/api
```

Now, you can access the web application at `http://localhost:8080`
