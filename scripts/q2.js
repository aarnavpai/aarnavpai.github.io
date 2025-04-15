function getTimestamp() {
  const date = new Date();
  return `${date.getFullYear()}-${date.getMonth()}-${date.getDate()} ${date.getHours()}:${date.getMinutes()}:${date.getSeconds()}`
}

document.addEventListener("DOMContentLoaded", () => {
  console.log(getTimestamp(), "view", "html")
})

document.addEventListener("click", (e) => {
  let name = (() => {
    const element = e.target
    const name = element.tagName.toLowerCase()
    switch(name) {
      case "img":
        return "image"
      case "p":
      case "h1":
      case "h2":
      case "h3":
      case "h4":
      case "h5":
      case "h6":
      case "span":
        return "text"
      case "select":
        return "drop-down"
    }
    if (element.className === 'theme-dropdown') return "drop-down"
    return name
  })()
  console.log(getTimestamp(), "click", name)
})

