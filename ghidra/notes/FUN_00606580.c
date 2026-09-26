
void __fastcall FUN_00606580(undefined4 *param_1)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_00656586;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0066e060;
  puVar1 = (undefined4 *)param_1[0x28];
  local_4 = 1;
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
    param_1[0x28] = 0;
  }
  puVar1 = (undefined4 *)param_1[0x41];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
    param_1[0x41] = 0;
  }
  puVar1 = (undefined4 *)param_1[0x2a];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
    param_1[0x2a] = 0;
  }
  if ((HWND)param_1[0x43] != (HWND)0x0) {
    SetFocus((HWND)param_1[0x43]);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_0060f460(param_1 + 0x3e);
  local_4 = 0xffffffff;
  FUN_005ffb60(param_1);
  ExceptionList = local_c;
  return;
}

