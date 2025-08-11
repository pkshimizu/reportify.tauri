import { Activity } from '@/models/activity';
import { invoke } from '@tauri-apps/api/core';

export default function useActivities() {
  return {
    fetchGitHubEvents: async () => {
      await invoke('fetch_github_events');
    },
    loadActivities: async (year: number, month: number) => {
      const activities = await invoke('load_activities', { year, month });
      return activities as Activity[];
    },
  };
}
