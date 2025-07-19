import { createFileRoute } from '@tanstack/react-router';
import RText from '../components/display/text';
import { RCenter } from '../components/layout/flex-box';

export const Route = createFileRoute('/settings')({
  component: Settings,
});

function Settings() {
  return (
    <RCenter>
      <RText>Settings</RText>
    </RCenter>
  );
}
