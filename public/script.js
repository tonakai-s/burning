console.log("start")
//const extensionId = "0553e9fd46c73875710d9f2877befc3c1366e396@temporary-addon"; // Replace with your actual extension ID
//chrome.runtime.sendMessage(extensionId, { type: "FROM_WEBPAGE", data: "Hello from the webpage!" }, function(response) {
//    if (response && response.status === "success") {
//        console.log("Message sent successfully to extension.");
//    } else {
//        console.error("Failed to send message to extension.");
//    }
//});

window.postMessage({
    direction: "from-page-script",
    message: "Message from the page"
  }, "*");
