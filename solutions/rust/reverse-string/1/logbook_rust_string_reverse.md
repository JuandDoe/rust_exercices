# Exercice Rust — Inverser une string

**Date :** 2026-06-09
**Exercice :** String reversal (Exercism)
**Niveau :** Débutant / Intermédiaire

---

## Objectif

Inverser une string en Rust, y compris pour des cas Unicode complexes comme `uüu`.

---

## Ce que j'ai compris

- Une `String` en Rust est un tableau d'**octets UTF-8**, pas de `char`. Inverser les bytes bruts casse l'encodage.
- Un `char` Rust représente un **scalaire Unicode** (valeur entre `0` et `0x10FFFF`). `.chars().rev()` fonctionne pour la majorité des cas.
- Mais certains caractères comme `ü` peuvent être représentés par **deux scalaires** : un `u` + un *combining diacritic*. Dans ce cas, `.chars().rev()` inverse l'ordre et casse le caractère.
- La bonne unité est le **grapheme cluster** : ce qu'un humain perçoit comme un seul caractère.
- La crate `unicode-segmentation` expose `.graphemes(true)` qui segmente une string en grapheme clusters.

## Solution finale

```rust
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let grapheme_cluster_input = input.graphemes(true).collect::<Vec<&str>>();
    grapheme_cluster_input.into_iter().rev().collect::<Vec<&str>>().join("")
}
```

Note : une version plus idiomatique serait `input.graphemes(true).rev().collect()` — mais la solution ci-dessus fonctionne et reflète le cheminement réel.

---

## Pièges rencontrés

- `todo!()` termine l'exécution — tout ce qui suit est du code mort.
- PascalCase sur une variable (`GraphemeClusterInput`) : Rust attend du `snake_case`.
- `.chars()` appelé sur un `Vec` : cette méthode est pour `&str`, pas pour un `Vec`.
- `.join()` n'est pas disponible sur un itérateur `Rev<T>` : il faut `.collect()` d'abord.
- `.iter()` sur un `Vec<&str>` produit un itérateur sur `&&str` — utiliser `.into_iter()` à la place.
- Retour de valeur en Rust : pas de `return`, pas de `let`, pas de `;` — l'expression seule suffit.

---

## À retenir / axes d'amélioration

- **La chaîne d'itérateurs Rust** est le vrai point de blocage ici. `.iter()` / `.into_iter()` / `.rev()` / `.collect()` / `.join()` — comprendre ce que chaque maillon produit comme type est fondamental. C'est un investissement à faire tôt.
- **Lire les messages d'erreur du compilateur jusqu'au bout** : sur le bug `&&str`, la réponse était littéralement dans le message. Réflexe à ancrer.
- **Le retour implicite** en Rust déroute au début — pas de `return`, pas de `;`. À mémoriser une fois pour toutes.
- **Chercher en termes de haut niveau** (`grapheme cluster`, `unicode segmentation`) plutôt que bas niveau (`combining class`).
- La tendance à chercher un crate trop spécifique est à surveiller.

---

## Répétition espacée

**J-7** — Revenir et répondre sans aide :
- Quelle est la différence entre un byte, un `char`, et un grapheme cluster ?
- Pourquoi `.chars().rev()` ne suffit pas pour le bonus ?

**J-30** — Réécrire la fonction de zéro, sans regarder la solution :
- Trouver la bonne crate et la bonne méthode seul
- Enchaîner `.into_iter().rev().collect().join("")` sans tâtonner sur les types

**J-90** — Contrôle honnête :
- Saurais-tu expliquer à voix haute pourquoi `.iter()` produit `&&str` sur un `Vec<&str>` ?
- Est-ce que le retour implicite en Rust est devenu un réflexe naturel ?
