
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __fastcall FUN_005f8600(int *param_1)

{
  DWORD DVar1;
  int iVar2;
  int iVar3;
  
  DAT_006be3a4 = 0;
  DAT_006be354 = 0;
  FUN_005ae1e0();
  DVar1 = timeGetTime();
  param_1[0x34] = DVar1;
  param_1[0x35] = 0;
  param_1[9] = 0;
  DAT_006be39c = 0;
  (**(code **)(param_1[6] + 4))();
  FUN_005f5b20(param_1 + 0x66);
  FUN_005f5b20(param_1 + 0x69);
  param_1[0x25] = 0;
  param_1[0x24] = 0;
  FUN_005f5540(&DAT_006be340);
  FUN_005f5540(&DAT_006be330);
  (**(code **)(*param_1 + 4))();
  (**(code **)(param_1[3] + 4))();
  (**(code **)(param_1[0x1e] + 4))();
  (**(code **)(param_1[0x21] + 4))();
  if (param_1[0x42] != 0) {
    if (param_1[0x43] != -1) {
      FUN_005ae1e0();
      (**(code **)(*(int *)param_1[0x42] + 0x24))((int *)param_1[0x42],param_1[param_1[0x43] + 0xd])
      ;
      param_1[0x43] = -1;
    }
    if ((HANDLE)param_1[0xc] != (HANDLE)0x0) {
      CloseHandle((HANDLE)param_1[0xc]);
      param_1[0xc] = 0;
    }
    (**(code **)param_1[0x91f])();
    iVar2 = (**(code **)(*(int *)param_1[0x42] + 0x10))((int *)param_1[0x42]);
    iVar3 = (**(code **)(*(int *)param_1[0x42] + 8))((int *)param_1[0x42]);
    param_1[0x42] = 0;
    _DAT_006be3ac = 0;
    if ((iVar2 != 0) && (iVar3 == 0)) {
      return 0;
    }
  }
  return 1;
}

