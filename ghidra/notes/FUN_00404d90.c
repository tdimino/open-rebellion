
void __fastcall FUN_00404d90(undefined4 *param_1)

{
  undefined *puVar1;
  undefined4 *puVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_00629796;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_00658580;
  local_4 = 1;
  if (param_1[0x27] != 0) {
    FUN_00602010(param_1[0x27]);
    if ((undefined4 *)param_1[0x27] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x27])(1);
    }
    param_1[0x27] = 0;
  }
  DAT_006b06f4 = 0;
  puVar1 = (undefined *)param_1[0x28];
  if (puVar1 != (undefined *)0x0) {
    FUN_00601990((int)puVar1);
    FUN_00618b60(puVar1);
  }
  FUN_00404ec0((int)param_1);
  puVar2 = (undefined4 *)param_1[0x34];
  if (puVar2 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar2);
    FUN_00618b60((undefined *)puVar2);
  }
  puVar2 = (undefined4 *)param_1[0x35];
  param_1[0x34] = 0;
  if (puVar2 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar2);
    FUN_00618b60((undefined *)puVar2);
  }
  puVar2 = (undefined4 *)param_1[0x36];
  param_1[0x35] = 0;
  if (puVar2 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar2);
    FUN_00618b60((undefined *)puVar2);
  }
  param_1[0x36] = 0;
  if (param_1[0x40] == 0) {
    FUN_00610c00(0);
  }
  (**(code **)(param_1[0x39] + 4))();
  local_4 = local_4 & 0xffffff00;
  FUN_00601540(param_1 + 0x39);
  local_4 = 0xffffffff;
  FUN_005ff250(param_1);
  ExceptionList = pvStack_c;
  return;
}

