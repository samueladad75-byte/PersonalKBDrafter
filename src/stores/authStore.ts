import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface AuthStore {
  jiraConnected: boolean;
  confluenceConnected: boolean;
  ollamaConnected: boolean;
  jiraSiteUrl: string | null;
  confluenceSiteUrl: string | null;
  setJiraConnected: (connected: boolean, siteUrl?: string) => void;
  setConfluenceConnected: (connected: boolean, siteUrl?: string) => void;
  setOllamaConnected: (connected: boolean) => void;
  disconnect: (service: 'jira' | 'confluence') => void;
}

export const useAuthStore = create<AuthStore>()(
  persist(
    (set) => ({
      jiraConnected: false,
      confluenceConnected: false,
      ollamaConnected: false,
      jiraSiteUrl: null,
      confluenceSiteUrl: null,

      setJiraConnected: (connected, siteUrl) =>
        set({ jiraConnected: connected, jiraSiteUrl: siteUrl ?? null }),

      setConfluenceConnected: (connected, siteUrl) =>
        set({ confluenceConnected: connected, confluenceSiteUrl: siteUrl ?? null }),

      setOllamaConnected: (connected) => set({ ollamaConnected: connected }),

      disconnect: (service) => {
        if (service === 'jira') {
          set({ jiraConnected: false, jiraSiteUrl: null });
        } else if (service === 'confluence') {
          set({ confluenceConnected: false, confluenceSiteUrl: null });
        }
      },
    }),
    {
      name: 'kb-drafter-auth',
    }
  )
);
