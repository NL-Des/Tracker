import { getConsent } from "./api.js";
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
      <button id="enter-btn"></button>
    </main>
  `;

  const localeSelect = root.querySelector("#locale-select");
  const title = root.querySelector("#home-title");
  const subtitle = root.querySelector("#home-subtitle");
  const button = root.querySelector("#enter-btn");

  function applyTranslations() {
    localeSelect.value = getLocale();
    title.textContent = t("home.title");
    subtitle.textContent = t("home.subtitle");
    button.textContent = t("home.enter");
  }

  applyTranslations();

  localeSelect.addEventListener("change", async () => {
    await setLocale(localeSelect.value);
    applyTranslations();
  });

  button.addEventListener("click", async () => {
    const consent = await getConsent();
    console.log("Consentement courant :", consent);
    // Point d'ancrage pour le routage vers le panneau à onglets (étapes 5-8).
    // Pas de logique d'authentification à ce stade.
  });
}
