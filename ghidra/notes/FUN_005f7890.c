
bool FUN_005f7890(LPCSTR param_1)

{
  MCIERROR MVar1;
  undefined4 local_1c;
  undefined4 local_18;
  char *local_14;
  undefined4 local_10;
  undefined4 local_c;
  undefined4 local_8;
  undefined4 local_4;
  
  FUN_005f78f0(param_1);
  local_1c = 0;
  local_18 = 0;
  local_14 = s_avivideo_006ac288;
  local_10 = 0;
  local_c = 0;
  local_8 = 0;
  local_4 = 0;
  MVar1 = mciSendCommandA(0,0x803,0x2000,(DWORD_PTR)&local_1c);
  return MVar1 == 0;
}

