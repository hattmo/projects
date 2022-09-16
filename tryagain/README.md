# TRYAGAIN

## About

Retrys code that can throw an exception N number of times.

## Installation

```bash
npm i @hattmo/tryagain
```

## How to

```javascript
import tryagain from "../src/index";

// Returns a promise that resolves to the return from the body function.
// Will reject when the body function fails N times and throw the most recent error.
// Calling exit will escape the attemps early and throw the most recent error
// If a promise is returned in either the body or the error handler, it will wait until the Promise is resolved before continuing.
tryagain(5, (attempt) => {

    // Will run 5 times before actually throwing
    potentialThrowingFunction("Bad Input");

    if (attempt > 2) { // Contains the current attempt number
        trysomethingelse("Good Input");
    }
}, (error, attempt, exit) => {
    console.log(`Attempt number ${attempt} had the following error: ${error}`);
});
```

## Author

Designed and maintained by [Matthew Howard](https://www.linkedin.com/in/matthew-howard-4013ba87/).

Support me with a [donation](https://www.paypal.me/hattmo)!
