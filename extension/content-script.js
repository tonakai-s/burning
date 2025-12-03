window.addEventListener("message", event => {
    if (event.source !== window) return;
    if (event.data?.from !== "burning") return;

    browser.runtime.sendMessage(event.data);
});
