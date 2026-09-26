
MCIERROR __thiscall FUN_00611e90(int *param_1,int param_2)

{
  MCIERROR MVar1;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  MCIDEVICEID local_10;
  undefined4 local_c;
  undefined4 local_8;
  undefined4 local_4;
  
  MVar1 = 0;
  if (param_2 != 0) {
    *param_1 = param_2;
    local_14 = 0;
    local_10 = 0;
    local_8 = 0;
    local_4 = 0;
    local_c = 0x204;
    MVar1 = mciSendCommandA(0,0x803,0x3000,(DWORD_PTR)&local_14);
    if (MVar1 == 0) {
      local_20 = 0;
      local_18 = 0;
      param_1[1] = local_10;
      local_1c = 10;
      MVar1 = mciSendCommandA(local_10,0x80d,0x400,(DWORD_PTR)&local_20);
      if (MVar1 == 0) {
        param_1[2] = 1;
        return 0;
      }
      FUN_00611f40((int)param_1);
    }
  }
  return MVar1;
}

