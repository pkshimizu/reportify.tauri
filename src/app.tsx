import CssBaseline from '@mui/material/CssBaseline';
import {
  ThemeProvider as MuiThemeProvider,
  createTheme,
} from '@mui/material/styles';
import { RouterProvider, createRouter } from '@tanstack/react-router';

import { routeTree } from '@/routeTree.gen';
import useSettings from './hooks/settings';

const router = createRouter({ routeTree });

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router;
  }
}

export function AppContent() {
  const { theme } = useSettings();

  const muiTheme = createTheme({
    palette: {
      mode: theme,
    },
  });

  return (
    <MuiThemeProvider theme={muiTheme}>
      <CssBaseline />
      <RouterProvider router={router} />
    </MuiThemeProvider>
  );
}
