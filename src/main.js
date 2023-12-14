const container = document.getElementById('app');
let response = await fetch('http://127.0.0.1:3000/text');
let text = await response.json();
let { content } = text;
content = "<h1>" + content + "</h1>";
container.innerHTML = content;