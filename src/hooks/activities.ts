import { invoke } from '@tauri-apps/api/core';

export default function useActivities() {
  return {
    fetchGitHubEvents: async () => {
      await invoke('fetch_github_events');
    },
  };
}
