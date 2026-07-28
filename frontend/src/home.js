import { t, getLocale, setLocale } from "./i18n.js";
import { collectAndExport } from "./api.js";
import { renderScanHistory } from "./history.js";

export function renderHome(root) {
  root.innerHTML = `
    <main>
      <select id="locale-select">
        <option value="fr">Français</option>
        <option value="en">English</option>
      </select>
      <h1 id="home-title"></h1>
      <p id="home-subtitle"></p>
      <button id="home-collect"></button>
      <p id="home-collect-status"></p>
      <div id="home-history"></div>
    </main>
  `;

  const localeSelect = root.querySelector("#locale-select");
  const title = root.querySelector("#home-title");
  const subtitle = root.querySelector("#home-subtitle");
  const collectButton = root.querySelector("#home-collect");
  const status = root.querySelector("#home-collect-status");
  const historyContainer = root.querySelector("#home-history");

  const history = renderScanHistory(historyContainer);

  function applyTranslations() {
    localeSelect.value = getLocale();
    title.textContent = t("home.title");
    subtitle.textContent = t("home.subtitle");
    collectButton.textContent = t("home.collect");
    history.applyTranslations();
  }

  applyTranslations();

  localeSelect.addEventListener("change", async () => {
    await setLocale(localeSelect.value);
    applyTranslations();
  });

  collectButton.addEventListener("click", async () => {
    collectButton.disabled = true;
    status.textContent = t("home.collecting");
    try {
      // "." reproduit le comportement du CLI : écriture dans le répertoire
      // courant du processus (pas de sélecteur de dossier pour ce premier jet).
      const paths = await collectAndExport(["json", "markdown", "xml"], ".");
      status.textContent = `${t("home.collect.success")} ${paths.join(", ")}`;
      await history.refresh();
    } catch (e) {
      status.textContent = `${t("home.collect.error")} ${e}`;
    } finally {
      collectButton.disabled = false;
    }
  });
}
