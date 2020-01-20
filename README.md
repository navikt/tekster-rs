# tekster-rs

Et forsøk på å skrive om syfotekster!

Bakgrunnen av dette ligger i at naisd skrus av 2. februar 2020. Dette er en fin mulighet til å få syfotekster til å
bygge på GitHub Actions. Siden den originale syfotekster er bygd på `no.nav.sbl.api-app` (som ligger et sted langt bak
murer og grenser), er det like greit å bygge applikasjonen på nytt.

Den nye applikasjonen er delt i to:
* [tekster-rs](https://github.com/navikt/tekster-rs) som er selve applikasjonen. Denne er skrevet i Rust, og bygges
kun når en endring i logikken kreves. Denne blir publisert som et Dockerimage på GitHub.
* [syfotekster-rs](https://github.com/navikt/syfotekster-rs) som er hvor alle tekstene lever. Denne henter et Dockerimage
fra `tekster-rs` og kopierer inn tekstene som brukes i applikasjonen. Dette gjør at vi ikke trenger å bygge appen hver
gang vi endrer på tekstene. Deployment burde gå raskt slik.

Grunnen til at appen er delt i to slik er fordi `tekster-rs` tar opp mot 7-8 minutter å bygge uten cache.
Hvis logikk i grunn ikke endres særlig ofte og vi trenger ikke at applikasjonen bygges mer enn nødvendig!
