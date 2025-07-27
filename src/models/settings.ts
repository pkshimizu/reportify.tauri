export type Theme = 'light' | 'dark';
export interface Settings {
  theme: Theme;
  language: string;
}

export interface GitHubSettings {
  id: number;
  personalAccessToken: string;
}
