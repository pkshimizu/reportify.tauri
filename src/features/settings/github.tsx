import RText from '@/components/display/text';
import RAddIcon from '@/components/icons/add';
import RRemoveIcon from '@/components/icons/remove';
import RIconButton from '@/components/input/icon-button';
import RSelect from '@/components/input/select';
import RTextField from '@/components/input/text-field';
import { RColumn, RRow } from '@/components/layout/flex-box';
import RGrid from '@/components/layout/grid';
import { default as useGitHub } from '@/hooks/github';
import useSettings from '@/hooks/settings';
import { useState } from 'react';

export default function SettingsGithub() {
  const [personalAccessToken, setPersonalAccessToken] = useState('');
  const { githubs, createGithub, deleteGithub } = useSettings();
  const { repositories } = useGitHub();
  const [selectedRepository, setSelectedRepository] = useState<
    string | undefined
  >();

  const handleCreateGithub = () => {
    createGithub(personalAccessToken);
    setPersonalAccessToken('');
  };

  return (
    <RGrid columns={['240px', '1fr']} alignItems='start' rowGap={1}>
      <RText>Personal Access Token</RText>
      <RColumn gap={1}>
        <RGrid
          columns={['1fr', '40px']}
          rowGap={4}
          columnGap={2}
          alignItems='center'
        >
          <RTextField
            value={personalAccessToken}
            onChange={setPersonalAccessToken}
          />
          <RIconButton onClick={handleCreateGithub}>
            <RAddIcon />
          </RIconButton>
        </RGrid>
        {githubs.map(github => (
          <RGrid
            key={github.id}
            columns={['1fr', '40px']}
            rowGap={4}
            columnGap={2}
            alignItems='center'
          >
            <RTextField
              value={github.personalAccessToken}
              readonly
              onChange={value => createGithub(value)}
            />
            <RIconButton onClick={() => deleteGithub(github.id)}>
              <RRemoveIcon />
            </RIconButton>
          </RGrid>
        ))}
      </RColumn>
      <RText>Repositories</RText>
      <RColumn gap={1}>
        <RGrid
          columns={['1fr', '40px']}
          rowGap={4}
          columnGap={2}
          alignItems='center'
        >
          <RSelect
            value={selectedRepository}
            items={repositories.map(repository => ({
              value: repository.id.toString(),
              label: (
                <RRow gap={1} align='center'>
                  <RText>{repository.owner.username}</RText>
                  <RText>/</RText>
                  <RText>{repository.name}</RText>
                </RRow>
              ),
            }))}
            onChange={setSelectedRepository}
          />
          <RIconButton onClick={() => {}}>
            <RAddIcon />
          </RIconButton>
        </RGrid>
      </RColumn>
    </RGrid>
  );
}
