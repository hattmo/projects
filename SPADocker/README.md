# Single Page App Docker Container

This image injects the environment variables from the container into
      process.env of the client js runtime. Additionally it has common settings
      for SPAs that require all paths to route to index JS to help with things
      like React Router. Finally the environment variable PORT sets the
      listening port of the server.
      