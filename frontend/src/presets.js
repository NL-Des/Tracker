import { getPreset, saveConsent } from "./api.js";
import { setConsentState } from "./state.js";
import { t } from "./i18n.js";

const PRESET_NAMES = ["none", "minimum", "medium", "maximum"];

export function renderPresets(root) {
  root.innerHTML = `
    <main>
      <h1 id="presets-title"></h1>
      <p id="presets-subtitle"></p>
      <ul id="presets-list"></ul>
      <p id="presets-status"></p>
    </main>
  `;

  const title = root.querySelector("#presets-title");
  const subtitle = root.querySelector("#presets-subtitle");
  const list = root.querySelector("#presets-list");
  const status = root.querySelector("#presets-status");

  title.textContent = t("presets.title");
  subtitle.textContent = t("presets.subtitle");

  for (const name of PRESET_NAMES) {
    const item = document.createElement("li");
    const button = document.createElement("button");
    button.id = `preset-${name}`;
    button.textContent = t(`presets.${name}`);
    const description = document.createElement("p");
    description.textContent = t(`presets.${name}.description`);

    button.addEventListener("click", async () => {
      const config = await getPreset(name);
      await saveConsent(config);
      setConsentState(config);
      status.textContent = t("presets.saved");
    });

    item.append(button, description);
    list.append(item);
  }
}
