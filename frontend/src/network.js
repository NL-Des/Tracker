import { t } from "./i18n.js";

// Aucune logique de collecte réseau côté Rust n'existe encore : simple point
// d'ancrage de navigation, sans `NetworkConsent` ni contenu de remplissage
// (cf. plan_client.md étape 8).
export function renderNetwork(root) {
  root.innerHTML = `
    <main>
      <h1 id="network-title"></h1>
      <p id="network-message"></p>
    </main>
  `;

  root.querySelector("#network-title").textContent = t("network.title");
  root.querySelector("#network-message").textContent = t("network.coming_soon");
}
