# AJVMIDDLEWARE

## About

Middleware for express that validates json bodies in requests using ajv

## Installation

```bash

npm i @hattmo/ajvmiddleware

```

## Example

```node

import { Router } from "express";
import ajvmiddleware from "@hattmo/ajvmiddleware";

const schema = {
  $schema: "http://json-schema.org/draft-07/schema#",
  type: "object",
  required: [
    "name",
    "age",
  ],
  properties: {
    name: {
      type: "string",
    },
    age: {
      type: "integer",
    },
  },
};

const route = Router();

route.post("/submit", ajvmiddleware(schema), (req, _res, next) => {
  console.log(`${req.body.name} is ${req.body.age} years old`);
  next()
});

export default route;

```

## Author

Designed and maintained by [Matthew Howard](https://www.linkedin.com/in/matthew-howard-4013ba87/).

Support me with a [donation](https://www.paypal.me/hattmo)!
