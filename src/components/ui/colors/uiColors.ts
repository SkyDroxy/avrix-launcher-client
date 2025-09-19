// Central shared UI color maps for interactive input components.
// This allows consistent theming across checkbox, switch, etc.
// Extend carefully; keep keys minimal and semantic.

export interface InteractiveColorSet {
  on: string; // background when active/on/checked
  off: string; // background when off/unchecked (or base)
  ring: string; // focus-visible ring color classes
  border?: string; // optional explicit border class
  thumb?: string; // thumb color (for switches)
  check?: string; // check icon / mark color
}

export const interactiveColors: Record<string, InteractiveColorSet> = {
  indigo: {
    on: 'bg-indigo-600',
    off: 'bg-neutral-600/60',
    ring: 'peer-focus-visible:ring-indigo-400/60',
    border: 'border-indigo-400/60',
    thumb: 'bg-white',
    check: 'text-indigo-50',
  },
  emerald: {
    on: 'bg-emerald-600',
    off: 'bg-neutral-600/60',
    ring: 'peer-focus-visible:ring-emerald-400/60',
    border: 'border-emerald-400/60',
    thumb: 'bg-white',
    check: 'text-emerald-50',
  },
  amber: {
    on: 'bg-amber-500',
    off: 'bg-neutral-600/60',
    ring: 'peer-focus-visible:ring-amber-400/60',
    border: 'border-amber-400/60',
    thumb: 'bg-white',
    check: 'text-amber-50',
  },
  rose: {
    on: 'bg-rose-600',
    off: 'bg-neutral-600/60',
    ring: 'peer-focus-visible:ring-rose-400/60',
    border: 'border-rose-400/60',
    thumb: 'bg-white',
    check: 'text-rose-50',
  },
  neutral: {
    on: 'bg-neutral-500',
    off: 'bg-neutral-700/70',
    ring: 'peer-focus-visible:ring-neutral-400/50',
    border: 'border-neutral-400/50',
    thumb: 'bg-neutral-100',
    check: 'text-neutral-50',
  },
};

export type InteractiveColorName = keyof typeof interactiveColors;
