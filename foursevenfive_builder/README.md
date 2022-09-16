# Four Seventy Five Builder

## About

This project is intended to ease the process of generating massive amounts of 475 forms for training and education units in the AF.  Please PR or contact me (matthew@hattmo.com) if you have any issues or suggestions.

## Requirements

To run this code you need [pdftk][1] in your path.

## Getting Started

- Install dependencies

  ```npm installl```

- Edit the `input.csv` file with all of the variable data.  Each row generates 1 pdf.
- Edit the `af475.pdf` with any data that is the same for every person.
- Adjust the code in `src/index.ts` to build the `Input` object as necessary. See the comments for more details.

## Running

```npm start```

A progress bar will indicate when the PDFs have been generated correctly. All PDFs will be placed in out/.

## Contact

Designed and maintained by [Matthew Howard][2].

Support me with a [donation][3]!

[1]: https://www.pdflabs.com/tools/pdftk-the-pdf-toolkit/
[2]: https://www.linkedin.com/in/matthew-howard-4013ba87/
[3]: https://www.paypal.me/hattmo
