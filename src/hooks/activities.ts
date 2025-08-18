import { Activity } from '@/models/activity';
import { invoke } from '@tauri-apps/api/core';
import { useCallback } from 'react';
import { useActivitiesState } from './activities-state';

export default function useActivities() {
  const { fetching, setFetching } = useActivitiesState();

  const fetchGitHubEvents = useCallback(async () => {
    setFetching(true);
    try {
      await invoke('fetch_github_events');
    } finally {
      setFetching(false);
    }
  }, [setFetching]);

  const fetchGitHubEventsWithRange = useCallback(
    async (startDate: string, endDate: string) => {
      setFetching(true);
      try {
        await invoke('fetch_github_events_with_range', { startDate, endDate });
      } finally {
        setFetching(false);
      }
    },
    [setFetching]
  );

  const loadActivities = useCallback(async (year: number, month: number) => {
    const activities = await invoke('load_activities', { year, month });
    return activities as Activity[];
  }, []);

  return {
    fetching,
    fetchGitHubEvents,
    fetchGitHubEventsWithRange,
    loadActivities,
  };
}
