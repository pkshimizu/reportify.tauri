import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';
import RTab from '../components/navigation/tab';
import SettingsGeneral from '../features/settings/general';
import SettingsGithub from '../features/settings/github';

export const Route = createFileRoute('/settings')({
  component: Settings,
});

function Settings() {
  const [tab, setTab] = useState('general');

  return (
    <RTab
      items={[
        { label: 'General', value: 'general', panel: <SettingsGeneral /> },
        {
          label: 'GitHub',
          value: 'github',
          panel: <SettingsGithub />,
        },
      ]}
      value={tab}
      orientation='vertical'
      onChange={setTab}
    />
  );
}
