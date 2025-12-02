self.addEventListener('message', event => {
	console.log('would start a socket')
});
//browser.runtime.onMessage.addListener((message, sender, sendResponse) => {
//    console.log("message received", message)
//    sendResponse({response: "msg recv by extension" })
//})
//browser.runtime.onMessageExternal.addListener(
//      function(request, sender, sendResponse) {
//        if (request.type === "FROM_WEBPAGE") {
//          console.log("Message received from webpage:", request.data);
//          // Process the message and send a response back to the webpage if needed
//          sendResponse({ status: "success", message: "Received your message!" });
//        }
//        // Return true to indicate that you want to send a response asynchronously
//        return true;
//      }
//    );
