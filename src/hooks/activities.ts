import { Activity } from '@/models/activity';
import { invoke } from '@tauri-apps/api/core';
import { useCallback } from 'react';

export default function useActivities() {
  const fetchGitHubEvents = useCallback(async () => {
    await invoke('fetch_github_events');
  }, []);

  const loadActivities = useCallback(async (year: number, month: number) => {
    const activities = await invoke('load_activities', { year, month });
    return activities as Activity[];
  }, []);

  return {
    fetchGitHubEvents,
    loadActivities,
  };
}
