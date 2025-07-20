import { invoke } from '@tauri-apps/api/core';
import {
  ReactNode,
  createContext,
  useContext,
  useEffect,
  useState,
} from 'react';
import { Theme } from '../models/theme';

interface ThemeContextType {
  theme: Theme;
  setTheme: (theme: Theme) => Promise<void>;
  isLoading: boolean;
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

export function useTheme() {
  const context = useContext(ThemeContext);
  if (context === undefined) {
    throw new Error('useTheme must be used within a ThemeProvider');
  }
  return context;
}

interface ThemeProviderProps {
  children: ReactNode;
}

export function ThemeProvider(props: ThemeProviderProps) {
  const [theme, setThemeState] = useState<Theme>('light');
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const loadCurrentTheme = async () => {
      try {
        const currentTheme = await invoke<string>('get_theme');
        setThemeState(currentTheme.toLowerCase() as Theme);
      } catch (error) {
        console.error('Failed to load current theme:', error);
        setThemeState('light');
      } finally {
        setIsLoading(false);
      }
    };

    loadCurrentTheme();
  }, []);

  const setTheme = async (newTheme: Theme) => {
    try {
      await invoke('update_theme', { theme: newTheme });
      setThemeState(newTheme);
    } catch (error) {
      console.error('Failed to update theme:', error);
    }
  };

  const value: ThemeContextType = {
    theme,
    setTheme,
    isLoading,
  };

  return (
    <ThemeContext.Provider value={value}>
      {props.children}
    </ThemeContext.Provider>
  );
}
