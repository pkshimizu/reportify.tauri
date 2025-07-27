import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { GitHubSettings } from '../models/settings';

export default function useSettings() {
  const [githubs, setGithubs] = useState<GitHubSettings[]>([]);

  useEffect(() => {
    const loadGithubs = async () => {
      const githubs = await invoke<GitHubSettings[]>('load_githubs');
      setGithubs(githubs);
    };
    loadGithubs();
  }, []);

  return {
    githubs,
    createGithub: async (personalAccessToken: string) => {
      const github = await invoke<GitHubSettings>('create_github', {
        personalAccessToken,
      });
      setGithubs([...githubs, github]);
    },
    deleteGithub: async (id: number) => {
      await invoke('delete_github', { id });
      setGithubs(githubs.filter(github => github.id !== id));
    },
  };
}
