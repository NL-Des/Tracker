import { getConsent } from "./api.js";

export function renderHome(root) {
  root.innerHTML = `
    <main>
      <h1>tracker</h1>
      <p>Contrôlez ce qui est collecté et partagé sur cette machine.</p>
      <button id="enter-btn">Commencer</button>
    </main>
  `;

  const button = root.querySelector("#enter-btn");
  button.addEventListener("click", async () => {
    const consent = await getConsent();
    console.log("Consentement courant :", consent);
    // Point d'ancrage pour le routage vers le panneau à onglets (étapes 5-8).
    // Pas de logique d'authentification à ce stade.
  });
}
