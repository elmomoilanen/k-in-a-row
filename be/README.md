# Backend

This is the backend service of the game, which computes the bot player's next move on the game board using the minimax algorithm with alpha-beta pruning.

## Development

Run `cargo run` to start the server.

By default, the server is bound to 0.0.0.0:8080, but this can be modified. The following command shows how to start the server bound to a standard IPv4 loopback address

```bash
ADDR=127.0.0.1 PORT=8080 cargo run
```

In principle, it is easy to add new game boards. A board must have the same number of rows and columns (i.e., a k x k board) and that's about the only strict requirement. Place proper board size parameters in `src/conf.rs` and the new board is ready to be used. Of course, the drawback for larger boards is that the search space for bot player's moves increases exponentially.

Server implements an endpoint `/api/bot/next` that accepts HTTP POST requests with a JSON type payload and a URL query string `level=VALUE` with allowed values of *Easy* and *Normal*.

The following example shows a valid request using the command line tool *curl* to compute the first move of a normal level 3x3 3-in-a-row game for the bot player

```bash
curl "localhost:8080/api/bot/next?level=Normal" \
    -H "content-type: application/json" \
    -d '{"cells":[0, 0, 0, 0, 0, 0, 0, 0, 0],"cells_to_win":3,"p1_mark":1,"bot_mark":-1,"empty_mark":0}'
```

and the response for it could be for example

```bash
{"next":0,"next_is_valid":true,"game_over":false,"winner":0}
```

where *next* indicates the board index for the bot's next move. Here indices must be interpreted such that 0-2 represent the first row of the 3x3 board, 3-5 the second row and 6-8 the third and last row.

For more information on the payload requirements, please see the model definitions in `src/models.rs`.

## Production

Document `gcp/README.md` gives instructions for deploying to Google Cloud Run.
