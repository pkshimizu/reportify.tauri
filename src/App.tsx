import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import {
  AppBar,
  Toolbar,
  Typography,
  Container,
  Box,
  TextField,
  Button,
  Card,
  CardContent,
  Stack,
  Chip,
} from '@mui/material';
import { Send as SendIcon } from '@mui/icons-material';
import reactLogo from './assets/react.svg';

function App() {
  const [greetMsg, setGreetMsg] = useState('');
  const [name, setName] = useState('');

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke('greet', { name }));
  }

  return (
    <Box sx={{ flexGrow: 1 }}>
      <AppBar position='static'>
        <Toolbar>
          <Typography variant='h6' component='div' sx={{ flexGrow: 1 }}>
            Reportify - Tauri + React + MUI
          </Typography>
        </Toolbar>
      </AppBar>

      <Container maxWidth='md' sx={{ mt: 4 }}>
        <Typography variant='h3' component='h1' gutterBottom align='center'>
          Welcome to Tauri + React
        </Typography>

        <Box sx={{ display: 'flex', justifyContent: 'center', gap: 2, mb: 4 }}>
          <Chip
            label='Vite'
            variant='outlined'
            avatar={
              <img
                src='/vite.svg'
                alt='Vite'
                style={{ width: 20, height: 20 }}
              />
            }
          />
          <Chip
            label='Tauri'
            variant='outlined'
            avatar={
              <img
                src='/tauri.svg'
                alt='Tauri'
                style={{ width: 20, height: 20 }}
              />
            }
          />
          <Chip
            label='React'
            variant='outlined'
            avatar={
              <img
                src={reactLogo}
                alt='React'
                style={{ width: 20, height: 20 }}
              />
            }
          />
        </Box>

        <Typography variant='body1' align='center' gutterBottom>
          Click on the Tauri, Vite, and React logos to learn more.
        </Typography>

        <Card sx={{ mt: 4 }}>
          <CardContent>
            <Typography variant='h5' component='h2' gutterBottom>
              Greet Function Test
            </Typography>

            <Box
              component='form'
              onSubmit={e => {
                e.preventDefault();
                greet();
              }}
              sx={{ mt: 2 }}
            >
              <Stack direction='row' spacing={2} alignItems='center'>
                <TextField
                  label='Enter a name'
                  variant='outlined'
                  value={name}
                  onChange={e => setName(e.target.value)}
                  placeholder='Enter a name...'
                  fullWidth
                />
                <Button
                  type='submit'
                  variant='contained'
                  endIcon={<SendIcon />}
                  sx={{ minWidth: 120 }}
                >
                  Greet
                </Button>
              </Stack>
            </Box>

            {greetMsg && (
              <Box sx={{ mt: 3 }}>
                <Typography variant='h6' color='primary'>
                  {greetMsg}
                </Typography>
              </Box>
            )}
          </CardContent>
        </Card>
      </Container>
    </Box>
  );
}

export default App;
