import { getConsent } from "./api.js";
import { t, getLocale, setLocale } from "./i18n.js";
import { setConsentState } from "./state.js";
import { renderPresets } from "./presets.js";

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
    setConsentState(consent);
    // Pas de logique d'authentification à ce stade : simple point d'ancrage.
    renderPresets(root);
  });
}
