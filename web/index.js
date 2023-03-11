const css = `
:host {
  display: flex;
  align-items: center;
  --counter-font: "Times New Roman";
  --counter-size: 40px;
  --counter-color: white;
  --counter-background: linear-gradient(to bottom, #999 0%, #ccc 40%, #999 100%);
  color: var(--counter-color);
  font-family: var(--counter-font);
  font-size: var(--counter-size);
}
span {
  border: 1px solid rgba(0, 0, 0, 0.5);
  padding: 5px;
  background: var(--counter-background);
  box-shadow: inset 0 0 0.125em white, inset 0 -0.125em 0.25em rgba(0, 0, 0, 0.5);
}
`

function createCounter(number) {
  number = number.toString();
  return [...number].map(char => {
    const node = document.createElement("span");
    node.innerText = char;
    return node;
  })
}

function uuidv4() {
  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
    let r = crypto.getRandomValues(new Uint8Array(1))[0] % 16;
    if (c === "y") {
      r = r & 0x3 | 0x8;
    }
    return r.toString(16);
  });
}

function getUserId() {
  let userId = localStorage.getItem("website-counter.userId")
  if (userId) {
    return userId
  }

  userId = uuidv4();
  localStorage.setItem("website-counter.userId", userId);
  return userId;
}

function getCookie(name) {
  var cookie = {};
  document.cookie.split(";").forEach(function(el) {
    var [key,value] = el.split("=");
    cookie[key.trim()] = value;
  })
  return cookie[name];
}

function getSessionId() {
  const sessionId = getCookie("session") || uuidv4()

  const now = new Date();
  const time = now.getTime();
  const expireTime = time + 1000 * 600; // let's keep this session for 10 min more
  now.setTime(expireTime);
  document.cookie = `session=${sessionId};expires=${now.toUTCString()};path=/`;

}

let cached;
let blocked = false;

function getData(host, show) {
  show = show || "visitors"
  const referrer = window.location.host;

  if (cached) {
    return new Promise((resolve) => {
      resolve(cached[show])
    })
  }

  if (blocked) {
    return new Promise(resolve => setTimeout(() => getData(host, show).then(resolve), 50))
  }

  blocked = true;
  return fetch(`${host}/count/${referrer}/${getUserId()}/${getSessionId()}`).then(r => r.json()).then((counter) => {
    cached = counter
    return counter[show]
  })
}

customElements.define("website-counter", class extends HTMLElement {
  constructor() {
    super();
    const shadowRoot = this.attachShadow({ mode: "open" });
    const style = document.createElement("style");
    style.textContent = css;
    shadowRoot.appendChild(style);
    this.counter = document.createElement("div")
    shadowRoot.appendChild(this.counter);
  }

  _showCount(show) {
    const host = this.getAttribute("url") || "https://website-counter.onrender.com";
    if (this.currentlyShown === show) return;
    this.currentlyShown = show;

    getData(host, show).then(result => {
      this.counter.textContent = "";
      this.counter.append(...createCounter(result))
    })
  }

  static get observedAttributes() {
    return ["show", "color", "size", "background", "font"];
  }

  connectedCallback() {
    this._showCount(this.getAttribute("show"))
  }

  attributeChangedCallback(name, _, newValue) {
    if (name === "show") {
      this._showCount(newValue)
    }
    if (name === "color") {
      this.style.setProperty("--counter-color", newValue);
    }
    if (name === "size") {
      this.style.setProperty("--counter-size", newValue);
    }
    if (name === "background") {
      this.style.setProperty("--counter-background", newValue);
    }
    if (name === "font") {
      this.style.setProperty("--counter-font", newValue);
    }
  }
});