import { GitHubRepository } from '@/models/github';
import { invoke } from '@tauri-apps/api/core';
import { useEffect } from 'react';
import { useGitHubState } from './github-state';

export default function useGitHub() {
  const { repositories, setRepositories } = useGitHubState();

  useEffect(() => {
    const loadRepositories = async () => {
      const repositories = await invoke<GitHubRepository[]>(
        'load_github_repositories'
      );
      setRepositories(repositories);
    };
    loadRepositories();
  }, [setRepositories]);

  return {
    repositories,
  };
}
