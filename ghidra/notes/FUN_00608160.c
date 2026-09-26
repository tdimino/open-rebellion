
void __fastcall FUN_00608160(undefined4 *param_1)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0065675f;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0066e0f0;
  param_1[0x25] = &PTR_LAB_0066e0c0;
  local_4 = 2;
  if (((*(byte *)(param_1 + 0x3d) & 8) == 0) && ((undefined4 *)param_1[0x28] != (undefined4 *)0x0))
  {
    (*(code *)**(undefined4 **)param_1[0x28])(1);
  }
  puVar1 = (undefined4 *)param_1[0x29];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x2b];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x2a];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x44];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x41];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_0060db40(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_0060a460((undefined4 *)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 0x25)));
  local_4 = 0xffffffff;
  FUN_005ffb60(param_1);
  ExceptionList = local_c;
  return;
}

