document.querySelector("button").addEventListener("click", () => {
    window.postMessage({ from: "burning", action: "reload" });
})

setTimeout(() => {
    const evtSource = new EventSource("/sse");
    console.log(evtSource);
    evtSource.onmessage = event => {
        console.log('New message from SSE: ' + event);
    }
    evtSource.addEventListener("chat_msg", (ev) => {
        console.log("New event: ", ev);
    });
}, 1000);