let consent = null;
const listeners = new Set();

export function getConsentState() {
  return consent;
}

export function setConsentState(next) {
  consent = next;
  for (const listener of listeners) listener(consent);
}

export function subscribe(listener) {
  listeners.add(listener);
  return () => listeners.delete(listener);
}
