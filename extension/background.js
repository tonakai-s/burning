browser.runtime.onMessage.addListener(async (msg, sender) => {
    console.log("message received from: ", sender);
    console.log("message:", msg);
    const tabs = await browser.tabs.query({currentWindow: true});
    let tab = tabs.find(t => t.url.includes(msg.target));
    if(tab){
        await browser.tabs.reload(tab.id, { bypassCache: true });
        await browser.tabs.sendMessage(sender.tab.id, "tab found and reload triggered");
    } else {
        await browser.tabs.sendMessage(sender.tab.id, "unable to found a tab with the provided target");
    }
});
