# Asset Loader

Asset Loader is a simple tool for defining and loading static assets for your project.  Store your static assets like pictures, videos, music, models, etc. on a web accessable url like S3.  Define the location of each asset in your package.json and load them with ```npx loadAssets``` or adding a script to your package.json ```"scripts":{"build":"loadAssets"}```.  Assets are loaded into the directory "./assets" in the root directory of your project.

## Build status

## Installation

```bash
npm i @hattmo/assetloader
```

## How to use

To use add an assets property to your config.json

```json
{
  "bugs": {
    "url": "https://github.com/bob/myproject.gt/issues"
  },
  "homepage": "https://github.com/bob/myproject.gt#readme",
  "assets" : [
    {
      "filename":"myimage.png",
      "uri":"https://fileRepository.com/images/myimage.png"
    }
  ]
}
```

## Tests

Install test dependencies and test with:

```bash
git clone https://github.com/hattmo/assetLoader.git
npm install
npm test
```