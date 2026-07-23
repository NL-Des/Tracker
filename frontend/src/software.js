import { listSoftwareFields, saveConsent } from "./api.js";
import { getConsentState, setConsentState } from "./state.js";
import { t } from "./i18n.js";
import { renderNetwork } from "./network.js";

const GROUPS = [
  { id: "system", fields: ["os", "desktop_environment", "kernel_modules"] },
  { id: "processes_users", fields: ["processes", "users", "env_vars"] },
  { id: "applications", fields: ["installed_apps", "dev_runtimes", "fonts"] },
  { id: "services_tasks", fields: ["services", "failed_services", "scheduled_tasks", "autostart_entries"] },
  { id: "packages", fields: ["package_managers", "update_history"] },
  { id: "containers_vm", fields: ["docker_images", "docker_volumes", "virtual_machines", "podman_images", "podman_volumes"] },
  { id: "network", fields: ["network_connections", "proxy_config"] },
  { id: "security", fields: ["ssh_keys", "security_status"] },
];

export async function renderSoftware(root) {
  root.innerHTML = `
    <main>
      <h1 id="software-title"></h1>
      <p id="software-subtitle"></p>
      <nav id="software-groups"></nav>
      <div id="software-fields"></div>
      <button id="software-save"></button>
      <p id="software-status"></p>
      <button id="software-next"></button>
    </main>
  `;

  const title = root.querySelector("#software-title");
  const subtitle = root.querySelector("#software-subtitle");
  const groupsNav = root.querySelector("#software-groups");
  const fieldsContainer = root.querySelector("#software-fields");
  const saveButton = root.querySelector("#software-save");
  const status = root.querySelector("#software-status");
  const nextButton = root.querySelector("#software-next");

  title.textContent = t("software.title");
  subtitle.textContent = t("software.subtitle");
  saveButton.textContent = t("software.save");
  nextButton.textContent = t("software.next");
  nextButton.addEventListener("click", () => renderNetwork(root));

  const fields = await listSoftwareFields();
  const knownFields = GROUPS.flatMap((group) => group.fields);
  const knownSet = new Set(knownFields);
  const receivedSet = new Set(fields);
  const isConsistent =
    knownFields.length === knownSet.size &&
    fields.length === receivedSet.size &&
    knownSet.size === receivedSet.size &&
    knownFields.every((field) => receivedSet.has(field));

  if (!isConsistent) {
    status.textContent = t("software.inconsistent");
    return;
  }

  const draft = structuredClone(getConsentState());

  function appendCheckbox(list, field, checked, onChange) {
    const item = document.createElement("li");
    const label = document.createElement("label");
    const checkbox = document.createElement("input");
    checkbox.type = "checkbox";
    checkbox.dataset.field = field;
    checkbox.checked = checked;
    checkbox.addEventListener("change", () => onChange(checkbox.checked));
    label.append(checkbox, t(`software.field.${field}`));
    item.append(label);
    list.append(item);
  }

  for (const group of GROUPS) {
    const groupButton = document.createElement("button");
    groupButton.dataset.group = group.id;
    groupButton.textContent = t(`software.group.${group.id}`);
    groupButton.addEventListener("click", () => showGroup(group.id));
    groupsNav.append(groupButton);

    const section = document.createElement("section");
    section.dataset.groupPanel = group.id;
    section.hidden = true;

    const list = document.createElement("ul");
    for (const field of group.fields) {
      appendCheckbox(list, field, Boolean(draft.software[field]), (checked) => {
        draft.software[field] = checked;
      });
    }
    // `browsers` vit dans ConsentConfig (pas dans SoftwareConsent) : rendu à part
    // dans "applications", hors de la vérification de parité des champs ci-dessus.
    if (group.id === "applications") {
      appendCheckbox(list, "browsers", Boolean(draft.browsers), (checked) => {
        draft.browsers = checked;
      });
    }
    section.append(list);
    fieldsContainer.append(section);
  }

  function showGroup(groupId) {
    for (const section of fieldsContainer.querySelectorAll("[data-group-panel]")) {
      section.hidden = section.dataset.groupPanel !== groupId;
    }
  }

  showGroup(GROUPS[0].id);

  saveButton.addEventListener("click", async () => {
    await saveConsent(draft);
    setConsentState(draft);
    status.textContent = t("software.saved");
  });
}
