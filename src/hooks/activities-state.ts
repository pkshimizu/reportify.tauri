import { create } from 'zustand';

interface ActivitiesState {
  fetching: boolean;
  setFetching: (fetching: boolean) => void;
}

export const useActivitiesState = create<ActivitiesState>(set => ({
  fetching: false,
  setFetching: (fetching: boolean) => set({ fetching }),
}));
