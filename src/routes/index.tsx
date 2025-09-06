import RText from '@/components/display/text';
import { RCenter, RColumn } from '@/components/layout/flex-box';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  return (
    <RCenter>
      <RColumn gap={2}>
        <RText>Welcome reportify</RText>
      </RColumn>
    </RCenter>
  );
}
