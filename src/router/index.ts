import LaunchView from '@components/launch/LaunchView.vue';
import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router';

// Lazy views (keep same component splitting semantics)
const PluginsView = () => import('@components/plugins/PluginsView.vue');
const VersionsView = () => import('@components/versions/VersionsView.vue');
const LogsView = () => import('@components/logs/LogsView.vue');
const PlayTestView = () => import('@components/playtest/PlayTestView.vue');

export const routes: RouteRecordRaw[] = [
  { path: '/', redirect: '/launch' },
  {
    path: '/launch',
    name: 'launch',
    component: LaunchView,
    meta: { label: 'Panneau de lancement' },
  },
  { path: '/plugins', name: 'plugins', component: PluginsView, meta: { label: 'Plugins' } },
  { path: '/versions', name: 'versions', component: VersionsView, meta: { label: 'Versions' } },
  { path: '/logs', name: 'logs', component: LogsView, meta: { label: 'Logs' } },
  {
    path: '/playtest',
    name: 'playtest',
    component: PlayTestView,
    meta: { label: 'PlayTest', devOnly: true },
  },
];

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export function routeLabel(name?: string | null) {
  if (!name) return '';
  const r = routes.find((r) => r.name === name);
  return (r?.meta as any)?.label || String(name);
}
