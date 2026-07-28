import { listSnapshots, openStorageLocation } from "./api.js";
import { t } from "./i18n.js";

export function renderScanHistory(root) {
  root.innerHTML = `
    <div id="history-header">
      <h2 id="history-title"></h2>
      <a id="history-open-file" href="#"></a>
    </div>
    <p id="history-status"></p>
    <div id="history-scroll" style="max-height: 240px; overflow-y: auto; border: 1px solid #ccc;">
      <table id="history-table" style="width: 100%; border-collapse: collapse;">
        <thead>
          <tr>
            <th id="history-col-id"></th>
            <th id="history-col-machine"></th>
            <th id="history-col-collected-at"></th>
          </tr>
        </thead>
        <tbody id="history-body"></tbody>
      </table>
    </div>
  `;

  const title = root.querySelector("#history-title");
  const openFileLink = root.querySelector("#history-open-file");
  const status = root.querySelector("#history-status");
  const colId = root.querySelector("#history-col-id");
  const colMachine = root.querySelector("#history-col-machine");
  const colCollectedAt = root.querySelector("#history-col-collected-at");
  const body = root.querySelector("#history-body");

  function applyTranslations() {
    title.textContent = t("history.title");
    openFileLink.textContent = t("history.open_file");
    colId.textContent = t("history.column.id");
    colMachine.textContent = t("history.column.machine");
    colCollectedAt.textContent = t("history.column.collected_at");
  }

  applyTranslations();

  openFileLink.addEventListener("click", async (event) => {
    event.preventDefault();
    await openStorageLocation();
  });

  async function refresh() {
    body.innerHTML = "";
    status.textContent = "";
    try {
      const snapshots = await listSnapshots();
      if (snapshots.length === 0) {
        status.textContent = t("history.empty");
        return;
      }
      for (const snapshot of snapshots) {
        const row = document.createElement("tr");

        const idCell = document.createElement("td");
        idCell.textContent = snapshot.id;

        const machineCell = document.createElement("td");
        machineCell.textContent = snapshot.host_name ?? snapshot.machine_id ?? "—";

        const collectedAtCell = document.createElement("td");
        collectedAtCell.textContent = new Date(snapshot.collected_at_unix * 1000).toLocaleString();

        row.append(idCell, machineCell, collectedAtCell);
        body.append(row);
      }
    } catch (e) {
      status.textContent = `${t("history.error")} ${e}`;
    }
  }

  refresh();

  return { applyTranslations, refresh };
}
