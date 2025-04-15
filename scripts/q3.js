(async () => {
  const PRONOUNS = (await (await fetch("/data/pronouns.txt")).text()).replace(/\n/g, " ");
  const PREPOSITIONS = (await (await fetch("/data/prepositions.txt")).text()).replace(/\n/g, " ").trim();

  document.getElementById("words").addEventListener("submit", e => {
    e.preventDefault();
    const fd = new FormData(e.target);
    const words = fd.get("words").toString();

    let nl = 0, spaces = 0, letters= 0, spec = 0;
    for (let i = 0; i < words.length; i++) {
      const char = words[i];
      if (char === '\n') nl++;
      else if (char === ' ') spaces++;
      else if (/[a-zA-z]/i.test(char)) letters++;
      else spec++;
    }
    const numWords = words.replaceAll("\n", " ").split(' ').filter(x => x.trim()).length;

    const result = document.getElementById("result");
    const ul = document.createElement("ul");
    result.innerHTML = "";
    ul.className = "calc";
    {
      const li = document.createElement("li");
      li.textContent = `Letters: ${letters}`;
      ul.appendChild(li);
    }
    {
      const li = document.createElement("li");
      li.textContent = `Words: ${numWords}`;
      ul.appendChild(li);
    }
    {
      const li = document.createElement("li");
      li.textContent = `New Lines: ${nl}`;
      ul.appendChild(li);
    }
    {
      const li = document.createElement("li");
      li.textContent = `Spaces: ${spaces}`;
      ul.appendChild(li);
    }
    {
      const li = document.createElement("li");
      li.textContent = `Special Characters: ${spec}`;
      ul.appendChild(li);
    }
    result.appendChild(ul);

    const pron = {};
    const prep = {};
    const art = {'a': 0, 'an': 0};

    for (const w of words.replace(/\n/g, ' ').split(' ')) {
      const word = w.trim();
      if (!word) continue;
      const re = new RegExp(`\\b${word}\\b`);
      if (re.test(PRONOUNS)) if (word in pron) pron[word]++; else pron[word] = 1;
      else if (re.test(PREPOSITIONS)) if (word in prep) prep[word]++; else prep[word] = 1;
      else if (word === 'a' || word === 'an') art[word]++;
    }

    for (const [name, nums] of [
      ["Pronouns", Object.entries(pron)],
      ["Prepositions", Object.entries(prep)],
      ["Indefinite Articles", Object.entries(art)],
    ]) {
      const div = document.createElement("div");
      const count = document.createElement("span");
      let sum = 0;
      const ul = document.createElement("ul");
      for (const [k, v] of nums) {
        if (v == 0) {
          continue;
        }
        sum += v;
        let li = document.createElement("li");
        li.textContent = `${k}: ${v}`;
        ul.appendChild(li);
      }
      count.textContent = `${name}: ${sum}`;
      div.appendChild(count);
      div.appendChild(ul);
      result.appendChild(div);
    }
  })
})()

