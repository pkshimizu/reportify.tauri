import RBox from '@/components/display/box';
import RText from '@/components/display/text';
import RGitHubIcon from '@/components/icons/github';
import { RCenter, RColumn, RRow } from '@/components/layout/flex-box';
import ActivityWeeklyCalendar from '@/features/activity/weekly-calendar';
import GitHubPullRequestList from '@/features/github/pull-request-list';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  return (
    <RCenter>
      <RBox m={2}>
        <RColumn gap={2}>
          <ActivityWeeklyCalendar />
          <RRow gap={1} align='center'>
            <RGitHubIcon />
            <RText>Open Pull Requests</RText>
          </RRow>
          <GitHubPullRequestList />
        </RColumn>
      </RBox>
    </RCenter>
  );
}
