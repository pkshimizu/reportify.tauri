import { createFileRoute } from '@tanstack/react-router';
import RText from '@/components/display/text';
import { RCenter } from '@/components/layout/flex-box';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  return (
    <RCenter>
      <RText>Welcome reportify</RText>
    </RCenter>
  );
}
