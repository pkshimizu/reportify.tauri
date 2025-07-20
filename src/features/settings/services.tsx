import RText from '../../components/display/text';
import RGrid from '../../components/layout/grid';

export default function SettingsServices() {
  return (
    <RGrid columns={['240px', '1fr']} alignItems='center'>
      <RText>GitHub</RText>
    </RGrid>
  );
}
