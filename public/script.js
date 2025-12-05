document.addEventListener("DOMContentLoaded", () => {
    const evtSource = new EventSource("/sse");
    console.log(evtSource);
    evtSource.onmessage = event => {
        console.log('New message from SSE: ' + event);
    }
    evtSource.addEventListener("ping", (ev) => {
        console.log("ping", ev);
    });
    evtSource.addEventListener("reload_from_file", (ev) => {
        console.log("event: ", ev);
        window.postMessage({ from: "burning", action: "reload", target: ev.data });
    });
});
