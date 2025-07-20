import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';
import RText from '../components/display/text';
import RTab from '../components/navigation/tab';

export const Route = createFileRoute('/settings')({
  component: Settings,
});

function Settings() {
  const [tab, setTab] = useState('general');

  return (
    <RTab
      items={[
        { label: 'General', value: 'general', panel: <RText>General</RText> },
        {
          label: 'Services',
          value: 'services',
          panel: <RText>Services</RText>,
        },
      ]}
      value={tab}
      orientation='vertical'
      onChange={setTab}
    />
  );
}
