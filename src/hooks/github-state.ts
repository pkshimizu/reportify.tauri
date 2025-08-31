import { GitHubRepository } from '@/models/github';
import { create } from 'zustand';

interface GitHubState {
  repositories: GitHubRepository[];
  setRepositories: (repositories: GitHubRepository[]) => void;
}

export const useGitHubState = create<GitHubState>(set => ({
  repositories: [],
  setRepositories: (repositories: GitHubRepository[]) => set({ repositories }),
}));
