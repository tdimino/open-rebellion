
undefined4 __fastcall FUN_00441d70(int *param_1)

{
  DWORD DVar1;
  uint uVar2;
  
  if ((param_1[0xf] == 0) && (param_1[0x10] == 0)) {
    DVar1 = timeGetTime();
    uVar2 = (DVar1 - param_1[0xe]) / (uint)param_1[0xd];
    if (uVar2 != param_1[0xc]) {
      if ((int)uVar2 < param_1[0xc]) {
        param_1[0xc] = uVar2;
        return 0;
      }
      (**(code **)(*param_1 + 0xc))(uVar2);
      return 1;
    }
  }
  return 0;
}

