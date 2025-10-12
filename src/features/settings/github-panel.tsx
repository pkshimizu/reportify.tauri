import RImage from '@/components/display/image';
import RTable from '@/components/display/table';
import RText from '@/components/display/text';
import RAddIcon from '@/components/icons/add';
import RRemoveIcon from '@/components/icons/remove';
import RButton from '@/components/input/button';
import RIconButton from '@/components/input/icon-button';
import RTextField from '@/components/input/text-field';
import { RColumn, RRow } from '@/components/layout/flex-box';
import RGrid from '@/components/layout/grid';
import RGridItem from '@/components/layout/grid-item';
import { useState } from 'react';
import GitHubRepositorySelectDialog from './github-repository-select-dialog';

export default function GitHubPanel() {
  const [privateAccessToken, setPrivateAccessToken] = useState<string>('');
  const [openRepositorySelectDialog, setOpenRepositorySelectDialog] =
    useState(false);

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
      <RRow gap={1} align='center'>
        <RImage
          src='https://avatars.githubusercontent.com/u/300403?v=4'
          alt='pkshimizu'
          width={20}
          height={20}
          circle
        />
        <RText>pkshimizu</RText>
        <RText>2025-12-31 12:34:56 expires</RText>
      </RRow>
      <RRow gap={1} align='center'>
        <RText>Repositories</RText>
        <RIconButton onClick={() => setOpenRepositorySelectDialog(true)}>
          <RAddIcon size='small' />
        </RIconButton>
        <GitHubRepositorySelectDialog
          open={openRepositorySelectDialog}
          onClose={() => setOpenRepositorySelectDialog(false)}
        />
      </RRow>
      <RTable
        columns={[
          { name: 'owner', cell: 'Owner', align: 'left', width: 100 },
          { name: 'repository', cell: 'Repository', align: 'left' },
          { name: 'actions', align: 'center', width: 64 },
        ]}
        rows={[
          {
            id: '1',
            cells: {
              owner: 'pkshimizu',
              repository: 'reportify.tauri',
              actions: (
                <RIconButton>
                  <RRemoveIcon />
                </RIconButton>
              ),
            },
          },
        ]}
      />
    </RColumn>
  );
}
