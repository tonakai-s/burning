browser.runtime.onMessage.addListener(async (msg, sender) => {
    const tabs = await browser.tabs.query({currentWindow: true});
    let tab = tabs.find(t => t.url.includes("wikipedia.org"));
    if(tab){
        await browser.tabs.reload(tab.id, { bypassCache: true });
    }
    console.log("Got from page:", msg);
});
