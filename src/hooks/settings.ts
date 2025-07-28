import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { GitHubSettings, Settings, Theme } from '@/models/settings';
import { useSettingsState } from './settings-state';

export default function useSettings() {
  const [githubs, setGithubs] = useState<GitHubSettings[]>([]);
  const { theme, setTheme } = useSettingsState();

  useEffect(() => {
    const loadSettings = async () => {
      const settings = await invoke<Settings>('load_settings');
      setTheme(settings.theme.toLowerCase() as Theme);
    };
    loadSettings();

    const loadGithubs = async () => {
      const githubs = await invoke<GitHubSettings[]>('load_githubs');
      setGithubs(githubs);
    };
    loadGithubs();
  }, [setTheme]);

  return {
    theme,
    setTheme: async (newTheme: Theme) => {
      await invoke('save_theme', { theme: newTheme });
      setTheme(newTheme);
    },
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
