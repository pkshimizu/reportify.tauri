import React from 'react';
import ReactDOM from 'react-dom/client';

import { AppContent } from '@/app';
import { ThemeProvider } from '@/contexts/ThemeContext';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider>
      <AppContent />
    </ThemeProvider>
  </React.StrictMode>
);
