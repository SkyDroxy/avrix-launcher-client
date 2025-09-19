export type LogLevel = 'ERROR' | 'WARN' | 'INFO' | 'DEBUG';

export interface LogEntry {
  raw: string;
  level: LogLevel;
  timestamp?: Date;
}

export interface LaunchExitEvent {
  code: number;
}
