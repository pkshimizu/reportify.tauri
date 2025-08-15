import RText from '@/components/display/text';
import RGitHubIcon from '@/components/icons/github';
import { RColumn, RRow } from '@/components/layout/flex-box';
import { Activity } from '@/models/activity';

interface Props {
  activities: Activity[];
}

export default function ActivityCounts({ activities }: Props) {
  const counts: Record<string, number> = {};
  activities.forEach(activity => {
    const service = activity.service;
    if (!counts[service]) {
      counts[service] = 0;
    }
    counts[service]++;
  });

  return (
    <RRow gap={0.5}>
      <RColumn align='center'>
        <RGitHubIcon size='small' />
        <RText size='small'>{counts['github'] ?? 0}</RText>
      </RColumn>
    </RRow>
  );
}
