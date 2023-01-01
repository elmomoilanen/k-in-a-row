## Backend ##

Backend service for the game. Computes the bot player's next move in the game board. This is done by applying the minimax algorithm with alpha-beta pruning.

### Build and run ###

Following instructions contain the recommended way to start the backend service locally. To change the listener to which the server is bound (default is 0.0.0.0:8080), please see the file `src/bin/server.rs`.

First build an image from the Dockerfile

```bash
docker build -t k-in-a-row/be:latest .
```

and after that run the image so that the server becomes available inside a new running container

```bash
docker run -p 127.0.0.1:8080:8080 --rm k-in-a-row/be
```

With the previous command, port 8080 of the container is bound to the same port on 127.0.0.1 of the host machine and is not accessible from the outside.
