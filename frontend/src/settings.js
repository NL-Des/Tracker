import { getRemoteExportConfig, saveRemoteExportConfig } from "./api.js";
import { t } from "./i18n.js";

export function renderSettings(root) {
  root.innerHTML = `
    <main>
      <h1 id="settings-title"></h1>
      <p id="settings-subtitle"></p>
      <label>
        <input type="checkbox" id="settings-remote-enabled" />
        <span id="settings-remote-enabled-label"></span>
      </label>
      <label id="settings-remote-url-label" for="settings-remote-url"></label>
      <input type="url" id="settings-remote-url" />
      <label id="settings-remote-token-label" for="settings-remote-token"></label>
      <input type="password" id="settings-remote-token" autocomplete="off" />
      <button id="settings-save"></button>
      <p id="settings-status"></p>
    </main>
  `;

  const enabledCheckbox = root.querySelector("#settings-remote-enabled");
  const urlInput = root.querySelector("#settings-remote-url");
  const tokenInput = root.querySelector("#settings-remote-token");
  const status = root.querySelector("#settings-status");

  root.querySelector("#settings-title").textContent = t("settings.title");
  root.querySelector("#settings-subtitle").textContent = t("settings.subtitle");
  root.querySelector("#settings-remote-enabled-label").textContent = t("settings.remote.enabled");
  root.querySelector("#settings-remote-url-label").textContent = t("settings.remote.url");
  root.querySelector("#settings-remote-token-label").textContent = t("settings.remote.token");
  const saveButton = root.querySelector("#settings-save");
  saveButton.textContent = t("settings.save");

  (async () => {
    const config = await getRemoteExportConfig();
    enabledCheckbox.checked = config.enabled;
    urlInput.value = config.url;
    tokenInput.value = config.auth_token ?? "";
  })();

  saveButton.addEventListener("click", async () => {
    try {
      await saveRemoteExportConfig({
        enabled: enabledCheckbox.checked,
        url: urlInput.value.trim(),
        auth_token: tokenInput.value.trim() || null,
      });
      status.textContent = t("settings.saved");
    } catch (e) {
      status.textContent = `${t("settings.error")} ${e}`;
    }
  });
}
