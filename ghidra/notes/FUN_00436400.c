
void __fastcall FUN_00436400(int param_1)

{
  int iVar1;
  int iVar2;
  undefined4 unaff_EBP;
  undefined4 *puStack_30;
  undefined4 local_20;
  void *local_1c;
  undefined4 local_18;
  int local_14 [2];
  undefined4 *puStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d558;
  puStack_c = ExceptionList;
  puStack_30 = (undefined4 *)0x436426;
  ExceptionList = &puStack_c;
  FUN_004ece30(&local_18);
  local_4 = 0;
  puStack_30 = (undefined4 *)0x436437;
  FUN_004ece30(&local_1c);
  local_4._0_1_ = 1;
  puStack_30 = (undefined4 *)0x436445;
  FUN_0042dbe0(&local_20);
  puStack_30 = &local_18;
  local_4 = CONCAT31(local_4._1_3_,2);
  iVar1 = (**(code **)(**(int **)(param_1 + 0x10) + 0x40))(local_14);
  iVar2 = (**(code **)(**(int **)(param_1 + 0x14) + 0x40))(&local_18,&stack0xffffffdc);
  if (iVar1 != 0) {
    (**(code **)(**(int **)(param_1 + 0x14) + 0x48))(unaff_EBP,&stack0xffffffd8,iVar1,&puStack_30);
  }
  if (iVar2 != 0) {
    (**(code **)(**(int **)(param_1 + 0x10) + 0x48))(local_20,&stack0xffffffd4,iVar2,&puStack_30);
  }
  if ((iVar1 == 3) && (iVar2 == 3)) {
    *puStack_c = 1;
  }
  else {
    *puStack_c = 0;
  }
  local_14[0]._0_1_ = 1;
  FUN_00619730();
  local_14[0] = (uint)local_14[0]._1_3_ << 8;
  FUN_00619730();
  local_14[0] = -1;
  FUN_00619730();
  ExceptionList = local_1c;
  return;
}

