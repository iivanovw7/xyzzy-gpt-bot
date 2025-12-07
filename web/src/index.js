import css from "./index.module.css";

function initializeApp() {
    const root = document.getElementById("root");
    const container = document.createElement("div");

    container.className = css.container;

    const header = document.createElement("h1");

    header.textContent = "Lorem ipsum";
    header.className = css.header;

    const message = document.createElement("p");

    message.textContent = "Lorem ipsum";

    container.appendChild(header);
    container.appendChild(message);
    root.appendChild(container);
}

document.addEventListener("DOMContentLoaded", initializeApp);

if (module.hot) {
    module.hot.accept();
}
