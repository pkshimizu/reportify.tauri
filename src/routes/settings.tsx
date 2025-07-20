import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';
import RTab from '../components/navigation/tab';
import SettingsGeneral from '../features/settings/general';
import SettingsServices from '../features/settings/services';

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
          label: 'Services',
          value: 'services',
          panel: <SettingsServices />,
        },
      ]}
      value={tab}
      orientation='vertical'
      onChange={setTab}
    />
  );
}
