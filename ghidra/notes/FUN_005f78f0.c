
bool FUN_005f78f0(LPCSTR param_1)

{
  MCIDEVICEID MVar1;
  MCIERROR MVar2;
  
  MVar1 = mciGetDeviceIDA(param_1);
  if (MVar1 != 0) {
    mciSendCommandA(MVar1,0x804,0,(DWORD_PTR)&param_1);
  }
  MVar1 = mciGetDeviceIDA(s_avivideo_006ac288);
  MVar2 = mciSendCommandA(MVar1,0x804,0,(DWORD_PTR)&param_1);
  return MVar2 == 0;
}

