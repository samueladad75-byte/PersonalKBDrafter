import { useEffect } from 'react';
import { invoke } from '../lib/tauri';
import { useAuthStore } from '../stores/authStore';

export function ConnectionStatus() {
  const { jiraConnected, setJiraConnected, ollamaConnected, setOllamaConnected, confluenceConnected, setConfluenceConnected } = useAuthStore();

  useEffect(() => {
    // Check connection status on mount
    async function checkConnections() {
      try {
        const jiraStatus = await invoke<boolean>('get_jira_connection_status');
        setJiraConnected(jiraStatus);
      } catch (error) {
        console.error('Failed to check Jira status:', error);
      }

      // Check Ollama status
      try {
        let ollamaUrl = 'http://localhost:11434';
        const settingsJson = localStorage.getItem('kb-drafter-settings');
        if (settingsJson) {
          try {
            const settings = JSON.parse(settingsJson);
            ollamaUrl = settings.state?.ollamaUrl ?? 'http://localhost:11434';
          } catch (parseError) {
            console.error('Failed to parse settings from localStorage:', parseError);
            // Fall back to default URL
          }
        }

        const ollamaStatus = await invoke<boolean>('check_ollama_status', { ollamaUrl });
        setOllamaConnected(ollamaStatus);
      } catch (error) {
        console.error('Failed to check Ollama status:', error);
        setOllamaConnected(false);
      }

      // Check Confluence status
      try {
        const confluenceStatus = await invoke<boolean>('get_confluence_connection_status');
        setConfluenceConnected(confluenceStatus);
      } catch (error) {
        console.error('Failed to check Confluence status:', error);
        setConfluenceConnected(false);
      }
    }

    checkConnections();

    // Re-check every 60 seconds
    const interval = setInterval(checkConnections, 60000);
    return () => clearInterval(interval);
  }, [setJiraConnected, setOllamaConnected, setConfluenceConnected]);

  return (
    <div className="flex items-center gap-3">
      <div className="flex items-center gap-1.5">
        <div
          className={`w-2 h-2 rounded-full ${
            jiraConnected ? 'bg-green-500' : 'bg-gray-400'
          }`}
        />
        <span className="text-sm text-white">Jira</span>
      </div>

      <div className="flex items-center gap-1.5">
        <div
          className={`w-2 h-2 rounded-full ${
            confluenceConnected ? 'bg-green-500' : 'bg-gray-400'
          }`}
        />
        <span className="text-sm text-white">Confluence</span>
      </div>

      <div className="flex items-center gap-1.5">
        <div
          className={`w-2 h-2 rounded-full ${
            ollamaConnected ? 'bg-green-500' : 'bg-gray-400'
          }`}
        />
        <span className="text-sm text-white">Ollama</span>
      </div>
    </div>
  );
}
