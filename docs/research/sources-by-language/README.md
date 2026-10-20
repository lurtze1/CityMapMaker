# Sources by language

> Written 2026-10-20 by seven research agents, one per language region, and assembled by Claude
> (issue #37). A claim here is binding only when a DECISIONS.md entry cites it. Every
> non-English quote is given in the original with a translation labelled "(translation:
> Claude)"; those translations are not published translations. Claims seen only in search
> summaries are marked "(unverified)". Unit conversions show their factor and its source.

Purpose: Jeffrey was concerned that the earlier research (PRs #34, #35) leaned on English
sources, often Wikipedia, even for non-European towns, which biases the generator toward
European towns. Each agent here searched in the region's own languages, quoted originals, and
looked for measured numbers.

## Reports

| File | Languages | Main new material |
|---|---|---|
| `chinese.md` | Chinese | Wall circuit by administrative rank from 1,252 Qing gazetteer entries; gate-count distribution; Kaogong ji with its commentary; Tang and Yuan street and ward sizes |
| `japanese.md` | Japanese | Edo land shares and density by class; block and plot modules for 17 castle towns; Heian-kyō grid; post towns |
| `islamic.md` | Arabic, Persian, Ottoman Turkish, Turkish, French | Maliki building law in the original (street classes in cubits, dead-end ownership, overlooking); hisba rules; Raymond's densities and dead-end shares; Ottoman street-width codes |
| `south-asia.md` | Sanskrit, Tamil, Hindi, English (Indian sources) | Arthashastra, Manasara, Mayamata and Samarangana parameter tables; Ahmedabad pols; 1870s gazetteer densities |
| `iberian.md` | Spanish, Portuguese | The 1573 Ordinances in Spanish; measured Spanish American blocks; Portuguese long lots and upper/lower towns; Andalusi lanes; Castilian new towns; pre-Columbian numbers |
| `continental.md` | French, German, Italian, Dutch | Charter plot sizes (Freiburg); bastide binary blocks and market squares; Florentine new towns; Dutch densities 1400-1560; German market forms |
| `africa.md` | Arabic, French, Portuguese, Dutch, Italian | Tarikh al-Sudan, Ibn Battuta, Barbosa, Dapper in the original; Djenné, Agadez, Harar, Benin, Kilwa, Old Oyo numbers |

## What the originals change

1. **Several English-language claims were wrong or too simple.**
   - The Arthashastra's royal roads are 8 danda (about 13.5 m) in the Sanskrit; the common
     English "4 danda" is a translation artefact.
   - The Islamic seven-cubit rule is a dispute and encroachment rule, not a minimum for every
     lane; the law itself has a hierarchy of 4, 7, 9 and 20 cubits (about 2 to 10 m).
   - Islamic law protected privacy from overlooking, not light or height.
   - The 1573 Spanish Ordinances fix the plaza and two lot sizes, not block size or street
     width; those were local choices.
   - Japan's "narrow plots because of a frontage tax" is disputed; Kyoto taxed per house.
   - The "Zähringer town plan" is partly a twentieth-century idea, rejected by German historians.
   - Suzhou had six land gates by the Qing gazetteer, not five; Pingyao's wall is recorded.
2. **Dead-end lanes are a process, not a style.** Spanish archaeology reads Andalusi dead ends
   as the result of a town filling up over time, and new excavations show planned orthogonal
   Islamic suburbs. Raymond measures dead ends at 52%, 47% and 41% of street length in Fez,
   Cairo and Aleppo. A rule pack can model them as saturation, not as a fixed layout.
3. **Many traditions are parameter tables.** The Sanskrit treatises, the Kaogong ji
   commentary, the Ottoman codes and the Spanish Ordinances give explicit numbers: street
   classes, gate counts, wall heights, size by rank. These are the easiest rule packs to write,
   and they show the pack format needs units (danda, cubit, zira, vara, ken, li) with stated
   metre factors.
4. **Size follows rank, not only population.** Chinese wall circuits follow administrative rank
   (county about 2.5 km, prefecture about 5.2 km, median); the Kaogong ji commentary and the
   Manasara scale the city to the ruler's rank. A rule pack may need a `rank` input as well as
   population.
5. **Zoning by direction and ring is explicit outside Europe.** The Arthashastra assigns castes
   and trades to compass sectors; the Manasara to concentric rings; Japanese castle towns to
   class rings with measured shares (Edo 68% samurai, 16% temple, 16% commoner land). This
   supports the two constraint shapes named in `docs/research/settlement/README.md`.
6. **Wards have different shapes.** A Japanese ward owns both sides of a street; a Harar quarter
   belongs to a gate; an Agadez quarter keeps the outline of an old camp; an Ahmedabad pol is a
   gated cluster of 5 to 60 houses with its own well. A district is not always a block.
7. **Density varies far more than the European figures suggested.** Measured or derived: Edo
   commoner land about 670 per ha against samurai land about 170; Cairo about 360 overall and
   588 in the walled core (1798); Djenné 221; Ahmedabad districts 116 to 349 per ha (1872);
   Dutch towns 75 (1400) to 127 (1560) on average; Kaifeng 120 to 130 and Hangzhou about 210
   (Song, per hectare from per km²). Density belongs per culture, era and class in rule packs.
8. **Plots differ in kind, not only size.** English burgage (narrow, very deep), German charter
   plot (wide, later split), southern French binary block (shallow houses, a back gap under
   0.5 m), Portuguese long lot, Spanish quarter-block lot, Japanese machiya on a 20-ken depth,
   Islamic courtyard house. A rule pack chooses a plot model.
9. **Sources disagree with each other.** Kilwa is "all of wood" in Ibn Battuta and
   "stone and lime" in Barbosa 185 years later; Timbuktu is 10 to 12 thousand people in Caillié
   and 100,000 in UNESCO (a secondary page; no primary record for that figure is cited here).
   Recommendation from this research, not yet adopted: a spec states an
   era, and the generator does not average sources.

## Limits

- Several texts were read from OCR scans (Ibn al-Rami, Ibn Abdun); quotes from them need
  checking against a printed edition before a decision relies on them.
- Many scholarly platforms (CNKI, some J-STAGE PDFs, publisher sites) blocked automated access.
- Unit factors carry real uncertainty: the Chinese li is 500 or 576 m depending on source; the
  Mauryan angula applied to later Indian treatises is an assumption (about ±15%).
- No agent found measured courtyard-house plot sizes for an Islamic city, street widths for
  Tamil temple-town car streets, or Marathi and Urdu sources.
