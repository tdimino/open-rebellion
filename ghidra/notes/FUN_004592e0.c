
void __fastcall FUN_004592e0(undefined4 *param_1)

{
  undefined4 *puVar1;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_006306f2;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_00659f18;
  local_4 = 3;
  if ((undefined4 *)param_1[100] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[100])(1);
  }
  if ((undefined4 *)param_1[99] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[99])(1);
  }
  puVar1 = (undefined4 *)param_1[0x55];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x66];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_0060db40(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x67];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  if ((undefined *)param_1[0x57] != (undefined *)0x0) {
    FUN_00618b60((undefined *)param_1[0x57]);
  }
  local_4._0_1_ = 2;
  FUN_004f4380(param_1 + 0x61);
  local_4._0_1_ = 1;
  FUN_0060a810(param_1 + 0x5d);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_0060a810(param_1 + 0x59);
  local_4 = 0xffffffff;
  FUN_004ac2f0(param_1);
  ExceptionList = pvStack_c;
  return;
}

