
void __fastcall FUN_0049f340(undefined4 *param_1)

{
  undefined4 *puVar1;
  undefined *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_006373e2;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0065bd10;
  local_4 = 3;
  if ((undefined4 *)param_1[0x54] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x54])(1);
  }
  if ((undefined4 *)param_1[0x55] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x55])(1);
  }
  if ((undefined4 *)param_1[0x56] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x56])(1);
  }
  if ((undefined4 *)param_1[0x6d] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x6d])(1);
  }
  if ((undefined4 *)param_1[0x6e] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x6e])(1);
  }
  if ((undefined4 *)param_1[0x70] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x70])(1);
  }
  if ((undefined4 *)param_1[0x6f] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x6f])(1);
  }
  puVar1 = (undefined4 *)param_1[0x5c];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar2 = (undefined *)param_1[0x5d];
  if (puVar2 != (undefined *)0x0) {
    FUN_00601990((int)puVar2);
    FUN_00618b60(puVar2);
  }
  puVar2 = (undefined *)param_1[0x5e];
  if (puVar2 != (undefined *)0x0) {
    FUN_00601990((int)puVar2);
    FUN_00618b60(puVar2);
  }
  puVar1 = (undefined4 *)param_1[0x5a];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x5b];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00601990((int)(param_1 + 0x5f));
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004ac2f0(param_1);
  ExceptionList = local_c;
  return;
}

