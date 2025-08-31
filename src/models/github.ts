export interface GitHubRepository {
  id: number;
  name: string;
  fullName: string;
  description: string;
  htmlUrl: string;
  private: boolean;
  owner: GitHubUser;
}

export interface GitHubUser {
  id: number;
  username: string;
  avatarUrl: string;
}
