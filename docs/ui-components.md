# UI Components

Documentation rapide pour les composants d'inputs récemment ajoutés/refactorés.

## UiInput

Props:

- modelValue: string|number|null
- type: string (default: text)
- size: xs|sm|md|lg (default: sm)
- variant: solid|outline|ghost (default: solid)
- placeholder: string
- disabled: boolean
- invalid: boolean (applique styles d'erreur)
- icon-left / icon-right: nom d'icône mingcute
- clearable: affiche un bouton clear quand non vide
- clear-title: tooltip du bouton clear (default: Clear)
- dense: réduit la hauteur sans changer la taille de police

Events:

- update:modelValue
- enter
- focus
- blur

Usage:

```vue
<UiInput
  v-model="query"
  dense
  size="xs"
  variant="outline"
  icon-left="mingcute:search-2-fill"
  clearable
/>
```

## UiCheckbox

Props:

- modelValue: boolean
- size: xs|sm|md (default: sm)
- color: indigo|emerald|amber|rose|neutral (default: indigo)
- disabled: boolean
- indeterminate: boolean (état visuel partiel)
- dense: mode compact (boîte + label plus serrés)

Events:

- update:modelValue

Usage:

```vue
<UiCheckbox v-model="enabled" dense color="emerald">Activer</UiCheckbox>
```

## UiSwitch

Props:

- modelValue: boolean
- size: xs|sm|md|lg (default: sm)
- color: indigo|emerald|amber|rose|neutral
- disabled: boolean
- dense: réduit l'espacement label

Events:

- update:modelValue

Usage:

```vue
<UiSwitch v-model="auto" size="xs" color="indigo" dense>Auto-follow</UiSwitch>
```

## Couleurs centralisées

Les cartes de couleurs interactives sont définies dans `@components/ui/colors/uiColors.ts` via `interactiveColors`. Cela permet d'ajouter facilement un nouveau thème (ex: cyan) à un seul endroit.

Exemple d'extension:

```ts
interactiveColors.cyan = {
  on: 'bg-cyan-500',
  off: 'bg-neutral-600/60',
  ring: 'peer-focus-visible:ring-cyan-400/60',
  border: 'border-cyan-400/60',
  thumb: 'bg-white',
  check: 'text-cyan-50',
};
```

## Bonnes pratiques futures

- Extraire un composable `useInteractiveColor(name)` si plusieurs autres composants en ont besoin.
- Ajouter tests visuels/story si un environnement Storybook ou Histoire est introduit.
- Unifier la gestion d'états invalid/success pour Input / Switch / Checkbox.
- Ajouter un mode `loading` pour UiSwitch (squelette ou spinner mini dans le thumb).

---

Dernière mise à jour: (générée automatiquement)
