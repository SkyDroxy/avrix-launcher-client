import { ref } from 'vue';

export interface LogEntry {
  id: number;
  ts: number; // epoch ms
  level: 'info' | 'warn' | 'error' | 'success';
  source: string; // e.g. install, scan, backend
  message: string;
}

const entries = ref<LogEntry[]>([]);
let counter = 0;

export function useLogs() {
  function addLog(partial: Omit<LogEntry, 'id' | 'ts'> & { ts?: number }) {
    entries.value.push({
      id: ++counter,
      ts: partial.ts ?? Date.now(),
      level: partial.level,
      source: partial.source,
      message: partial.message,
    });
  }
  function clearLogs() {
    entries.value = [];
  }
  return { entries, addLog, clearLogs };
}
