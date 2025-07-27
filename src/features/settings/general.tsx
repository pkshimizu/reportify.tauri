import RText from '@/components/display/text';
import RDarkIcon from '@/components/icons/dark';
import RLightIcon from '@/components/icons/light';
import RToggleButton from '@/components/input/toggle-button';
import RGrid from '@/components/layout/grid';
import { useTheme } from '@/contexts/ThemeContext';
import { Theme } from '@/models/settings';

export default function SettingsGeneral() {
  const { theme, setTheme, isLoading } = useTheme();

  const handleThemeChange = (newTheme: string) => {
    setTheme(newTheme as Theme);
  };

  if (isLoading) {
    return <RText>Loading...</RText>;
  }

  return (
    <RGrid columns={['240px', '1fr']} alignItems='center'>
      <RText>Theme</RText>
      <RToggleButton
        items={[
          { value: 'light', content: <RLightIcon /> },
          { value: 'dark', content: <RDarkIcon /> },
        ]}
        value={theme}
        onChange={handleThemeChange}
      />
    </RGrid>
  );
}
