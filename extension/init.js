let socket = undefined;
let pingInterval = undefined;
const btn = document.getElementById("myButton");
btn.addEventListener('click', ev => {
	browser.tabs.create({ url: 'burning.html' });
});


//socket = new WebSocket("ws://127.0.0.1:9001");
//socket.addEventListener("open", () => {
//	console.log("connection openned");
//	let counter = 0;
//	pingInterval = setInterval(() => {
//		socket.send("ping " + counter);
//		counter++;
//	}, 1000);
//});
//socket.addEventListener("close", () => {
//	console.log("connection closed");
//});
//socket.addEventListener("message", (e) => {
//	console.log(e.data);
//});
//socket.addEventListener("error", (e) => {
//	console.log(e);
//});
