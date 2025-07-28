import { Theme } from '@/models/settings';
import { create } from 'zustand';

interface SettingsState {
  theme: Theme;
  setTheme: (theme: Theme) => void;
}

export const useSettingsState = create<SettingsState>(set => ({
  theme: 'light',
  setTheme: (theme: Theme) => set({ theme }),
}));
