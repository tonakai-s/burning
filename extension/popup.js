document.addEventListener("DOMContentLoaded", () => {
  const btn = document.getElementById("myButton");

  btn.addEventListener("click", () => {
    console.log("Button clicked!");
    alert("Button clicked!");
    fetch("http://127.0.0.1:8080/");
  });
});
