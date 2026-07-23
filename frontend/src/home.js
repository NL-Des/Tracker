import { t, getLocale, setLocale } from "./i18n.js";

export function renderHome(root) {
  root.innerHTML = `
    <main>
      <select id="locale-select">
        <option value="fr">Français</option>
        <option value="en">English</option>
      </select>
      <h1 id="home-title"></h1>
      <p id="home-subtitle"></p>
    </main>
  `;

  const localeSelect = root.querySelector("#locale-select");
  const title = root.querySelector("#home-title");
  const subtitle = root.querySelector("#home-subtitle");

  function applyTranslations() {
    localeSelect.value = getLocale();
    title.textContent = t("home.title");
    subtitle.textContent = t("home.subtitle");
  }

  applyTranslations();

  localeSelect.addEventListener("change", async () => {
    await setLocale(localeSelect.value);
    applyTranslations();
  });
}
