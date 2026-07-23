import { listHardwareFields, saveConsent } from "./api.js";
import { getConsentState, setConsentState } from "./state.js";
import { t } from "./i18n.js";
import { renderSoftware } from "./software.js";

const GROUPS = [
  { id: "storage", fields: ["disks", "virtual_disks", "storage_layout", "optical_drives"] },
  { id: "network", fields: ["network", "wifi", "bluetooth_devices"] },
  { id: "sensors", fields: ["components", "fans"] },
  { id: "power", fields: ["batteries", "power_profile"] },
  { id: "system", fields: ["cpu", "memory", "motherboard"] },
  { id: "display", fields: ["monitors", "gpus"] },
  { id: "bus", fields: ["pci_devices", "usb_devices"] },
  { id: "peripherals", fields: ["peripherals", "mice", "gamepads", "touchpads", "cameras", "printers"] },
];

export async function renderHardware(root) {
  root.innerHTML = `
    <main>
      <h1 id="hardware-title"></h1>
      <p id="hardware-subtitle"></p>
      <nav id="hardware-groups"></nav>
      <div id="hardware-fields"></div>
      <button id="hardware-save"></button>
      <p id="hardware-status"></p>
      <button id="hardware-next"></button>
    </main>
  `;

  const title = root.querySelector("#hardware-title");
  const subtitle = root.querySelector("#hardware-subtitle");
  const groupsNav = root.querySelector("#hardware-groups");
  const fieldsContainer = root.querySelector("#hardware-fields");
  const saveButton = root.querySelector("#hardware-save");
  const status = root.querySelector("#hardware-status");
  const nextButton = root.querySelector("#hardware-next");

  title.textContent = t("hardware.title");
  subtitle.textContent = t("hardware.subtitle");
  saveButton.textContent = t("hardware.save");
  nextButton.textContent = t("hardware.next");
  nextButton.addEventListener("click", () => renderSoftware(root));

  const fields = await listHardwareFields();
  const knownFields = GROUPS.flatMap((group) => group.fields);
  const knownSet = new Set(knownFields);
  const receivedSet = new Set(fields);
  const isConsistent =
    knownFields.length === knownSet.size &&
    fields.length === receivedSet.size &&
    knownSet.size === receivedSet.size &&
    knownFields.every((field) => receivedSet.has(field));

  if (!isConsistent) {
    status.textContent = t("hardware.inconsistent");
    return;
  }

  const draft = structuredClone(getConsentState());

  for (const group of GROUPS) {
    const groupButton = document.createElement("button");
    groupButton.dataset.group = group.id;
    groupButton.textContent = t(`hardware.group.${group.id}`);
    groupButton.addEventListener("click", () => showGroup(group.id));
    groupsNav.append(groupButton);

    const section = document.createElement("section");
    section.dataset.groupPanel = group.id;
    section.hidden = true;

    const list = document.createElement("ul");
    for (const field of group.fields) {
      const item = document.createElement("li");
      const label = document.createElement("label");
      const checkbox = document.createElement("input");
      checkbox.type = "checkbox";
      checkbox.dataset.field = field;
      checkbox.checked = Boolean(draft.hardware[field]);
      checkbox.addEventListener("change", () => {
        draft.hardware[field] = checkbox.checked;
      });
      label.append(checkbox, t(`hardware.field.${field}`));
      item.append(label);
      list.append(item);
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
    status.textContent = t("hardware.saved");
  });
}
