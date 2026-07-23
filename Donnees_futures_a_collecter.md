# Données futures à collecter — projet `tracker`

> Ce document liste les données **non collectées actuellement** mais potentiellement exploitables dans une future itération du projet, ainsi que les raisons pour lesquelles elles ne le sont pas encore (limitation OS, besoin de droits root/admin, hors périmètre volontaire...). Pour ce qui est déjà collecté, voir `donnees_collectees.md`.

## Matériel / système bas niveau
- Historique d'usage CPU/mémoire dans le temps (séries temporelles au lieu d'un instantané unique).
- Fréquence et latence RAM (timings), nombre de barrettes, emplacements.
- Santé disque S.M.A.R.T. détaillée (secteurs défectueux, durée de vie estimée, cycles d'écriture SSD) — seul le statut sommaire PASSED/FAILED sur NVMe est collecté ; SATA/ATA nécessite généralement root.
- Courbes de refroidissement des ventilateurs et marque/modèle (vitesse RPM déjà collectée ; le reste vit dans les tables SMBIOS type 27, nécessite root).
- Historique/courbe de charge de la batterie (dégradation dans le temps, pas juste une valeur instantanée).
- Qualité de connexion Wi-Fi détaillée (bande passante réelle) — SSID/force du signal/débit de liaison déjà collectés.
- Classification fine des périphériques USB (stockage/réseau/autre, via descripteurs d'interface) — seuls nom/fabricant sont collectés actuellement.
- Adresse IP publique (nécessiterait une requête sortante vers un service externe — hors périmètre pour l'instant, le projet reste sans dépendance réseau sortante ; adresse IP locale/passerelle/DNS déjà collectés).
- **Vitesse de liaison réseau et type de connexion sur macOS** — implémentés sur Linux/Windows, pas de source lecture-libre identifiée sur macOS pour l'instant.
- **RAID logiciel / LVM sur macOS et Windows** — implémentés sur Linux uniquement (`/proc/mdstat`, `lvs`) ; Storage Spaces sur Windows nécessiterait des classes WMI plus complexes.

## Logiciel / OS
- Historique de démarrage (crashs, temps de boot).
- Logs système récents (erreurs noyau, journaux d'événements) — nécessite généralement root pour les logs complets (`dmesg`, `journalctl` sans droits limité à la session courante).
- **Containerd** (`ctr`/`nerdctl`) — volontairement non couvert, son socket nécessite généralement root (Docker et Podman sont couverts).

## Données d'usage / comportement (nécessiterait un suivi dans le temps)
- Temps d'utilisation par application (pas seulement instantané CPU/mémoire).
- Historique de connexion/déconnexion utilisateur.
- Fréquence de lancement des applications.

> Volontairement exclu du périmètre pour rester non intrusif : historique du presse-papiers, liste des fichiers récemment ouverts, tout suivi fin de l'usage applicatif au-delà d'un instantané.

## Métadonnées / qualité de collecte
- Bilan structuré (et non une simple liste de messages texte) : pour chaque champ attendu, statut collecté/échoué + raison de l'échec (permissions insuffisantes, capteur absent, plateforme non supportée, etc.), plutôt que la liste actuelle de chaînes libres dans `collection_warnings`.

## Données externes (nécessiteraient une connexion réseau, actuellement absente du projet)
- Météo locale (via une API météo).
- Cours de cryptomonnaies/bourse (si pertinent pour un futur usage).
- Vérification de la version la plus récente disponible pour les applications installées (comparaison avec un registre en ligne).
- Géolocalisation approximative (IP → ville) pour enrichir le rapport.
- Vulnérabilités connues (CVE) pour les logiciels installés détectés.

## Sécurité / conformité
- État du pare-feu, chiffrement de disque, produit antivirus : déjà implémentés en best-effort sans admin (`security_status.rs`, voir `donnees_collectees.md`) — reste non couvert : règles de pare-feu détaillées, statut BitLocker sur Windows (nécessite une élévation en pratique malgré la documentation Microsoft).

## Nécessite systématiquement root/admin
- UUID machine (`/sys/class/dmi/id/product_uuid` sur Linux).
- `dmidecode` sur Linux (BIOS bas niveau, RAM détaillée si sysfs insuffisant).
- Historique boot/crash détaillé, logs noyau complets (`dmesg` complet).
- Liste de tous les ports ouverts par tous les utilisateurs (selon OS).
- Règles de pare-feu détaillées, statut BitLocker sur Windows (`manage-bde`/`Get-BitLockerVolume` exigent une élévation en pratique).

---

## Annexe technique — données accessibles sans privilèges root/admin

Points identifiés comme techniquement atteignables sans élévation, utiles pour prioriser de futurs développements :

- **`ProcessorId` CPU (Windows)** : via WMI `Win32_Processor`, pas d'admin requis.
- **Numéro de série de la carte mère (Windows)** : WMI `Win32_BaseBoard.SerialNumber` sans admin. (Sur Linux, `/sys/class/dmi/id/board_serial` est souvent restreint root-only selon la distribution — à vérifier au cas par cas.)
- **Numéro de série RAM (Windows)** : WMI `Win32_PhysicalMemory.SerialNumber` sans admin. (Sur Linux, `dmidecode -t 17` nécessite root ; pas d'équivalent sysfs non privilégié fiable.)
- **IP publique** (via une requête sortante) : aucune élévation requise, mais nécessiterait une connexion réseau sortante du projet (non implémenté).

> Note : le modèle/numéro de série des disques, le fabricant/modèle/numéro de série des écrans (EDID), le numéro de série de la batterie et l'adresse MAC des interfaces réseau sont déjà collectés sans élévation — voir `donnees_collectees.md`.
