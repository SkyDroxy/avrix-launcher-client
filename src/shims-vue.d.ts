// Global shims for importing .vue Single File Components in TypeScript
// This allows statements like: import App from './App.vue'
// and provides proper component typing for templates.
declare module '*.vue' {
  import type { DefineComponent } from 'vue';
  const component: DefineComponent<Record<string, unknown>, Record<string, unknown>, unknown>;
  export default component;
}

// (Optional) support importing static assets with typed modules if needed later:
// declare module '*.svg' { const url: string; export default url }
// declare module '*.png' { const url: string; export default url }
