document.querySelector("button").addEventListener("click", () => {
    window.postMessage({ from: "burning", action: "reload" });
})
