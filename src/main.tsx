import CssBaseline from '@mui/material/CssBaseline';
import {
  ThemeProvider as MuiThemeProvider,
  createTheme,
} from '@mui/material/styles';
import { RouterProvider, createRouter } from '@tanstack/react-router';
import React from 'react';
import ReactDOM from 'react-dom/client';

import { ThemeProvider, useTheme } from '@/contexts/ThemeContext';
import { routeTree } from '@/routeTree.gen';

const router = createRouter({ routeTree });

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router;
  }
}

function AppContent() {
  const { theme } = useTheme();

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

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider>
      <AppContent />
    </ThemeProvider>
  </React.StrictMode>
);
