import { invoke as tauriInvoke } from '@tauri-apps/api/core';

export interface AppError {
  kind: string;
  message: string;
}

export function handleInvokeError(error: unknown): AppError {
  if (typeof error === 'string') {
    try {
      return JSON.parse(error);
    } catch {
      return { kind: 'unknown', message: error };
    }
  }
  return { kind: 'unknown', message: String(error) };
}

// Type-safe invoke wrapper
export async function invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  return tauriInvoke<T>(command, args);
}
