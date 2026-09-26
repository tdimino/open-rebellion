
void __fastcall FUN_00612300(undefined4 *param_1)

{
  int *piVar1;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00656f3e;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066e3b8;
  local_4 = 2;
  (**(code **)(param_1[0x10] + 4))();
  piVar1 = (int *)param_1[0x13];
  if (piVar1 != (int *)0x0) {
    if (*(int *)(*(int *)(*(int *)(*piVar1 + 4) + 4 + (int)piVar1) + 0x4c) != -1) {
      FUN_006168a0(piVar1);
    }
    piVar1 = (int *)param_1[0x13];
    if (piVar1 != (int *)0x0) {
      (*(code *)**(undefined4 **)(*(int *)(*piVar1 + 4) + (int)piVar1))(1);
    }
  }
  FUN_006123b0((int)param_1);
  local_4._0_1_ = 1;
  FUN_00612ae0(param_1 + 0x10);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00612c60(param_1 + 8);
  local_4 = 0xffffffff;
  FUN_005f5c50(param_1);
  ExceptionList = pvStack_c;
  return;
}

