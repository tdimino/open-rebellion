
bool FUN_005f7730(undefined4 param_1,undefined4 param_2,MCIDEVICEID *param_3)

{
  MCIERROR MVar1;
  undefined4 local_44;
  undefined4 local_40;
  undefined4 local_3c;
  undefined4 local_38;
  undefined1 local_34 [8];
  undefined4 local_2c;
  undefined4 local_1c;
  MCIDEVICEID local_18;
  undefined4 local_14;
  undefined4 local_10;
  undefined4 local_c;
  undefined4 local_8;
  undefined4 local_4;
  
  local_1c = 0;
  local_18 = 0;
  local_14 = 0;
  local_10 = param_2;
  local_c = 0;
  local_8 = 0x40000000;
  local_4 = param_1;
  MVar1 = mciSendCommandA(0,0x803,0x30200,(DWORD_PTR)&local_1c);
  if (MVar1 == 0) {
    local_44 = 0;
    *param_3 = local_18;
    local_40 = param_1;
    local_3c = 5;
    local_38 = 0;
    MVar1 = mciSendCommandA(local_18,0x841,0x50000,(DWORD_PTR)&local_44);
    if (MVar1 == 0) {
      local_2c = 0x4001;
      MVar1 = mciSendCommandA(*param_3,0x814,0x100,(DWORD_PTR)local_34);
    }
  }
  return MVar1 == 0;
}

