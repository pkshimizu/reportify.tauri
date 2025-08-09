import RText from '@/components/display/text';
import RSyncIcon from '@/components/icons/sync';
import RIconButton from '@/components/input/icon-button';
import { RCenter, RColumn, RRow } from '@/components/layout/flex-box';
import useActivities from '@/hooks/activities';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  const { fetchGitHubEvents } = useActivities();

  const handleFetchGithubEvents = () => {
    fetchGitHubEvents();
  };

  return (
    <RCenter>
      <RColumn gap={2}>
        <RText>Welcome reportify</RText>
        <RRow justify='center'>
          <RIconButton onClick={handleFetchGithubEvents}>
            <RSyncIcon />
          </RIconButton>
        </RRow>
      </RColumn>
    </RCenter>
  );
}
