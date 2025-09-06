import { Theme } from '@/models/settings';
import { invoke } from '@tauri-apps/api/core';
import { useSettingsState } from './settings-state';

export default function useSettings() {
  const { theme, setTheme } = useSettingsState();

  return {
    theme,
    setTheme: async (newTheme: Theme) => {
      await invoke('save_theme', { theme: newTheme });
      setTheme(newTheme);
    },
  };
}
