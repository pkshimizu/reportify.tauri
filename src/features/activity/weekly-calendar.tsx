import RBox from '@/components/display/box';
import RText from '@/components/display/text';
import RGitHubIcon from '@/components/icons/github';
import { RColumn, RRow } from '@/components/layout/flex-box';
import RGrid from '@/components/layout/grid';

export default function ActivityWeeklyCalendar() {
  return (
    <RGrid columns={['1fr', '1fr', '1fr', '1fr', '1fr', '1fr', '1fr']} gap={1}>
      <RBox px={2} py={1} align='center' justify='center' bgcolor='weekday'>
        <RColumn align='center'>
          <RText color='weekday'>Mon</RText>
          <RText color='weekday'>1</RText>
          <RBox height={100} justify='center' align='center'>
            <RColumn>
              <RRow gap={1} align='center'>
                <RGitHubIcon />
                <RText>999</RText>
              </RRow>
            </RColumn>
          </RBox>
        </RColumn>
      </RBox>
      <RBox px={2} align='center' justify='center' bgcolor='weekday'>
        <RColumn align='center'>
          <RText color='weekday'>Tue</RText>
          <RText color='weekday'>2</RText>
          <RBox height={100} justify='center' align='center'>
            <RColumn>
              <RRow gap={1} align='center'>
                <RGitHubIcon />
                <RText>999</RText>
              </RRow>
            </RColumn>
          </RBox>
        </RColumn>
      </RBox>
      <RBox px={2} align='center' justify='center' bgcolor='weekday'>
        <RColumn align='center'>
          <RText color='weekday'>Wed</RText>
          <RText color='weekday'>3</RText>
          <RBox height={100} justify='center' align='center'>
            <RColumn>
              <RRow gap={1} align='center'>
                <RGitHubIcon />
                <RText>999</RText>
              </RRow>
            </RColumn>
          </RBox>
        </RColumn>
      </RBox>
      <RBox px={2} align='center' justify='center' bgcolor='weekday'>
        <RColumn align='center'>
          <RText color='weekday'>Thu</RText>
          <RText color='weekday'>4</RText>
          <RBox height={100} justify='center' align='center'>
            <RColumn>
              <RRow gap={1} align='center'>
                <RGitHubIcon />
                <RText>999</RText>
              </RRow>
            </RColumn>
          </RBox>
        </RColumn>
      </RBox>
      <RBox px={2} align='center' justify='center' bgcolor='weekday'>
        <RColumn align='center'>
          <RText color='weekday'>Fri</RText>
          <RText color='weekday'>5</RText>
          <RBox height={100} justify='center' align='center'>
            <RColumn>
              <RRow gap={1} align='center'>
                <RGitHubIcon />
                <RText>999</RText>
              </RRow>
            </RColumn>
          </RBox>
        </RColumn>
      </RBox>
      <RBox px={2} align='center' justify='center' bgcolor='sat'>
        <RColumn align='center'>
          <RText color='sat'>Sat</RText>
          <RText color='sat'>6</RText>
          <RBox height={100} justify='center' align='center'>
            <RColumn>
              <RRow gap={1} align='center'>
                <RGitHubIcon />
                <RText>999</RText>
              </RRow>
            </RColumn>
          </RBox>
        </RColumn>
      </RBox>
      <RBox px={2} align='center' justify='center' bgcolor='sun'>
        <RColumn align='center'>
          <RText color='sun'>Sun</RText>
          <RText color='sun'>7</RText>
          <RBox height={100} justify='center' align='center'>
            <RColumn>
              <RRow gap={1} align='center'>
                <RGitHubIcon />
                <RText>999</RText>
              </RRow>
            </RColumn>
          </RBox>
        </RColumn>
      </RBox>
    </RGrid>
  );
}
