
void __fastcall FUN_00437cb0(undefined4 *param_1)

{
  undefined *puVar1;
  int *piVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0062d664;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_00658db8;
  local_4 = 2;
  for (piVar2 = (int *)(**(code **)(param_1[0x46] + 8))(); piVar2 != (int *)0x0;
      piVar2 = (int *)(**(code **)(*piVar2 + 0xc))()) {
    puVar1 = (undefined *)piVar2[0x15];
    if (puVar1 != (undefined *)0x0) {
      FUN_00619730();
      FUN_00618b60(puVar1);
    }
  }
  ReleaseCapture();
  if ((undefined4 *)param_1[0x45] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x45])(1);
  }
  if ((undefined4 *)param_1[0x4a] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x4a])(1);
  }
  puVar1 = (undefined *)param_1[0x4b];
  if (puVar1 != (undefined *)0x0) {
    FUN_00601990((int)puVar1);
    FUN_00618b60(puVar1);
  }
  puVar1 = (undefined *)param_1[0x4c];
  if (puVar1 != (undefined *)0x0) {
    FUN_00601990((int)puVar1);
    FUN_00618b60(puVar1);
  }
  puVar1 = (undefined *)param_1[0x4d];
  if (puVar1 != (undefined *)0x0) {
    FUN_00601990((int)puVar1);
    FUN_00618b60(puVar1);
  }
  puVar1 = (undefined *)param_1[0x4e];
  if (puVar1 != (undefined *)0x0) {
    FUN_00601990((int)puVar1);
    FUN_00618b60(puVar1);
  }
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_0060a810(param_1 + 0x46);
  local_4 = 0xffffffff;
  FUN_00606580(param_1);
  ExceptionList = pvStack_c;
  return;
}

