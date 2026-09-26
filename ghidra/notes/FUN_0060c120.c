
void __fastcall FUN_0060c120(undefined4 *param_1)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_00656ad6;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0066e1c8;
  local_4 = 1;
  if (param_1[0x3a] != 0) {
    FUN_00600280(param_1[0x3a]);
    if ((undefined4 *)param_1[0x3a] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x3a])(1);
    }
    param_1[0x3a] = 0;
  }
  if ((HGDIOBJ)param_1[0x2c] != (HGDIOBJ)0x0) {
    DeleteObject((HGDIOBJ)param_1[0x2c]);
  }
  puVar1 = (undefined4 *)param_1[0x3c];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x3d];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_0060a810(param_1 + 0x34);
  local_4 = 0xffffffff;
  FUN_005ffb60(param_1);
  ExceptionList = local_c;
  return;
}

