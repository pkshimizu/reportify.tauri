import RTable from '@/components/display/table';
import RText from '@/components/display/text';
import RButton from '@/components/input/button';
import RTextField from '@/components/input/text-field';
import { RColumn } from '@/components/layout/flex-box';
import RGrid from '@/components/layout/grid';
import RGridItem from '@/components/layout/grid-item';
import { useState } from 'react';

export default function GitHubPanel() {
  const [privateAccessToken, setPrivateAccessToken] = useState<string>('');
  return (
    <RColumn fullWidth gap={1}>
      <RGrid columns={['auto', '1fr', 'auto']} gap={1} alignContent='start'>
        <RGridItem align='center'>
          <RText>Private Access Token</RText>
        </RGridItem>
        <RGridItem align='center'>
          <RTextField
            value={privateAccessToken}
            fullWidth
            onChange={setPrivateAccessToken}
          />
        </RGridItem>
        <RGridItem align='center'>
          <RButton variant='outlined' onClick={() => setPrivateAccessToken('')}>
            Save
          </RButton>
        </RGridItem>
      </RGrid>
      <RText>Target Repositories</RText>
      <RTable
        columns={[
          { name: 'Owner', align: 'left', width: 100 },
          { name: 'Repository', align: 'left', width: 200 },
        ]}
        rows={[{ id: '1', cells: { Owner: 'Repo 1', Repository: 'Repo 1' } }]}
      />
    </RColumn>
  );
}
