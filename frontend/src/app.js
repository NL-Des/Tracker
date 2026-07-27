import { renderHome } from "./home.js";
import { renderPresets } from "./presets.js";
import { renderHardware } from "./hardware.js";
import { renderSoftware } from "./software.js";
import { renderNetwork } from "./network.js";
import { renderSettings } from "./settings.js";
import { t } from "./i18n.js";

const TABS = [
  { id: "home", render: renderHome, labelKey: "tabs.home" },
  { id: "presets", render: renderPresets, labelKey: "presets.title" },
  { id: "hardware", render: renderHardware, labelKey: "hardware.title" },
  { id: "software", render: renderSoftware, labelKey: "software.title" },
  { id: "network", render: renderNetwork, labelKey: "network.title" },
  { id: "settings", render: renderSettings, labelKey: "settings.title" },
];

// Chaque page est montée une seule fois (comme de vrais onglets de navigateur) :
// changer d'onglet bascule juste la visibilité, ça ne recrée jamais le contenu,
// donc les modifications non enregistrées survivent à la navigation.
export function renderApp(root) {
  root.innerHTML = `
    <nav id="tab-bar"></nav>
    <div id="tab-panels"></div>
  `;

  const tabBar = root.querySelector("#tab-bar");
  const panelsContainer = root.querySelector("#tab-panels");

  function showTab(tabId) {
    for (const button of tabBar.querySelectorAll("[data-tab]")) {
      button.setAttribute("aria-selected", String(button.dataset.tab === tabId));
    }
    for (const panel of panelsContainer.querySelectorAll("[data-tab-panel]")) {
      panel.hidden = panel.dataset.tabPanel !== tabId;
    }
  }

  for (const tab of TABS) {
    const button = document.createElement("button");
    button.dataset.tab = tab.id;
    button.textContent = t(tab.labelKey);
    button.addEventListener("click", () => showTab(tab.id));
    tabBar.append(button);

    const panel = document.createElement("section");
    panel.dataset.tabPanel = tab.id;
    panel.hidden = true;
    panelsContainer.append(panel);
    tab.render(panel);
  }

  showTab(TABS[0].id);
}
