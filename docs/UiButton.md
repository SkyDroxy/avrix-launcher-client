# UiButton

Composant centralisé pour tous les boutons de l'interface. Il fournit une API cohérente pour les variantes, tailles, états et comportements (focus ring, animation de pression, chargement, icône seule, etc.).

## Variantes

- `primary` (action principale)
- `secondary` (action secondaire neutre)
- `accent` (mise en avant alternative / accent de marque)
- `info` (action neutre informative)
- `success` (confirmation / réussite)
- `warning` (attention, action potentiellement risquée)
- `danger` (destructive / erreur)
- `ghost` (faible emphase / commandes contextuelles)

## Tailles

- `xs` (icônes, actions denses, toolbars)
- `sm` (par défaut compact)
- `md` (par défaut « standard ») – valeur par défaut
- `lg` (actions larges / call-to-action)

## Props

| Prop        | Type    | Default             | Description                                                                       |
| ----------- | ------- | ------------------- | --------------------------------------------------------------------------------- |
| `variant`   | string  | `primary`           | Variante visuelle (voir liste).                                                   |
| `size`      | string  | `md`                | Taille du bouton.                                                                 |
| `type`      | string  | `button`            | Type HTML (`button`, `submit`, `reset`).                                          |
| `block`     | boolean | `false`             | Force `w-full` et empile le contenu verticalement si besoin.                      |
| `square`    | boolean | `false`             | Rend le bouton carré (utilisé pour icônes seules).                                |
| `disabled`  | boolean | `false`             | Désactive le bouton.                                                              |
| `loading`   | boolean | `false`             | Affiche un spinner et désactive le clic.                                          |
| `icon`      | boolean | `false`             | Indique un bouton icône seule (ajuste padding/alignement).                        |
| `iconRight` | boolean | `false`             | Place une icône secondaire à droite.                                              |
| `border`    | boolean | `true` (sauf ghost) | Force l'affichage (ou suppression) de la bordure.                                 |
| `focusRing` | boolean | `true`              | Active le ring de focus (accessibilité). Désactiver pour faux boutons décoratifs. |
| `press`     | boolean | `true`              | Active l'animation de pression (scale).                                           |

## États

- `:disabled` applique opacité réduite et empêche les interactions.
- `loading` superpose un spinner (icône ou contenu caché via opacité/visibilité).

## Icônes

Utiliser le slot par défaut avec un composant `Icon`. Pour un bouton icône seule :

```vue
<UiButton variant="ghost" size="xs" square icon :aria-label="'Fermer'">
  <Icon name="mingcute:close-line" :width="16" />
</UiButton>
```

## Exemples

### Action principale

```vue
<UiButton @click="submit">Enregistrer</UiButton>
```

### Bouton destructif

```vue
<UiButton variant="danger" @click="remove" :disabled="busy">Supprimer</UiButton>
```

### Bouton avec état de chargement

```vue
<UiButton :loading="loading" @click="perform">Traitement...</UiButton>
```

### Bouton fantôme discret

```vue
<UiButton variant="ghost" size="sm" @click="open">Options</UiButton>
```

### Désactivation du ring / animation (ex: sidebar)

```vue
<UiButton variant="ghost" :focusRing="false" :press="false" class="justify-start w-full">
  <Icon name="mingcute:home-3-fill" :width="18" />
  <span>Accueil</span>
</UiButton>
```

## Bonnes pratiques

- Toujours fournir `aria-label` pour un bouton icône seule.
- Préférer `primary` pour l'action principale unique d'un bloc ou modal.
- Utiliser `danger` uniquement pour les actions destructives (suppression, reset irréversible).
- Limiter l'usage de `accent` à une mise en valeur alternative (éviter la compétition visuelle avec `primary`).
- `ghost` est idéal pour les actions secondaires, contextuelles ou peu prioritaires.

## Migration Legacy

- `IconButton` supprimé (remplacé par `UiButton square variant="ghost"`).
- `GlowButton` encapsule désormais `UiButton` (style spécial conservé). Pour une unification future, on pourra transformer l'effet glow en variante dédiée.

## Accessibilité

- Conserver `focusRing=true` dès qu'une navigation clavier est attendue.
- Remplacer `:focusRing="false"` uniquement si un style visuel alternatif de focus est fourni ou si le bouton est purement décoratif.

## TODO Futur (optionnel)

- Variantes supplémentaires: `outline`, `link`.
- Gestion auto `focus-visible` (au lieu de simple focus) pour réduire le bruit visuel.
- Prop `as` pour rendre `<a>` ou autre élément tout en gardant le style.
