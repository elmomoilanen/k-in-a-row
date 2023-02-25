# Backend

Backend service for the game. Computes the bot player's next move in the game board. This is done by applying the minimax algorithm with alpha-beta pruning.

## Build and run ##

Following instructions contain the recommended way to start the backend service locally.

First build an image from the Dockerfile

```bash
docker build -t k-in-a-row/be:latest .
```

and after that run the image so that the server becomes available inside a new running container

```bash
docker run -p 127.0.0.1:8080:8080 --rm k-in-a-row/be
```

With the previous command, port 8080 of the container (server's default port) is bound to the same port on 127.0.0.1 of the host machine and is not accessible from the outside.

## Use ##

Server implements one endpoint `/api/bot/next` and only accepts POST requests with a JSON type payload and a URL query string `level=VALUE` with allowed values of *Easy* and *Normal*.

Following example shows a valid HTTP request (using curl) used to compute the first move of a normal level 3x3 3-in-a-row game for the bot player

```bash
curl "localhost:8080/api/bot/next?level=Normal" \
    -H "content-type: application/json" \
    -d '{"cells":[0, 0, 0, 0, 0, 0, 0, 0, 0],"p1_mark":1,"bot_mark":-1,"empty_mark":0}'
```

and the response for it could be e.g.

```bash
{"next":0,"next_is_valid":true,"game_over":false,"winner":0}
```

where *next* indicates the board index for the bot's next move. Here indices must be interpreted such that 0-2 represent the first row of the 3x3 board, 3-5 the second row and etc.

For more information on the payload requirements, please see the model definitions in `src/models.rs`.

## Development ##

Run `cargo run` to start the server.

By default the server is bound to 0.0.0.0:8080 but during development one may want to modify these values. Following command shows how to start the server bound to different address

```bash
ADDR=127.0.0.1 PORT=8000 cargo run
```

In principle, it is easy to add new game boards. A board must have a same number of rows and columns (i.e., a k x k board) and that's about the only strict requirement. Place proper board size values in `src/game.rs` and `src/bot.rs` by following the example of earlier game boards. Of course, the drawback for larger boards is that the search space for bot player's moves increases exponentially.
