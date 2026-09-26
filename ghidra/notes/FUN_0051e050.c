
undefined4 __fastcall FUN_0051e050(int param_1)

{
  int iVar1;
  bool bVar2;
  void *pvVar3;
  void *pvVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  undefined4 uVar9;
  int iVar10;
  undefined3 extraout_var;
  undefined4 local_1c [2];
  undefined4 local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643200;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (*(int *)(param_1 + 0x48) != 0) {
    iVar10 = *(int *)(param_1 + 0x6c);
    iVar1 = *(int *)(param_1 + 0x88);
    ExceptionList = &local_c;
    pvVar3 = FUN_0051e1e0((void *)(param_1 + 0x6c),local_14);
    local_4 = 0;
    pvVar4 = FUN_0051e1e0((void *)(param_1 + 0x88),local_1c);
    local_4._0_1_ = 1;
    uVar5 = FUN_005f6030((int)pvVar3);
    uVar6 = FUN_005f6030((int)pvVar4);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_005f5fe0(local_1c);
    local_4 = 0xffffffff;
    FUN_005f5fe0(local_14);
    pvVar3 = FUN_0051e200((void *)(param_1 + 0x6c),local_1c);
    local_4 = 2;
    pvVar4 = FUN_0051e200((void *)(param_1 + 0x88),local_14);
    local_4._0_1_ = 3;
    uVar7 = FUN_005f6030((int)pvVar4);
    uVar8 = FUN_005f6030((int)pvVar3);
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_005f5fe0(local_14);
    local_4 = 0xffffffff;
    FUN_005f5fe0(local_1c);
    if ((uVar5 == uVar6) && (iVar10 == iVar1)) {
      uVar9 = 0;
    }
    else {
      uVar9 = 1;
    }
    *(undefined4 *)(param_1 + 0x30) = uVar9;
    if (*(int *)(param_1 + 0x4c) == 0) {
      *(uint *)(param_1 + 0x38) = (uint)(uVar8 != uVar7);
    }
    else {
      *(uint *)(param_1 + 0x40) = (uint)(uVar8 != uVar7);
    }
    *(undefined4 *)(param_1 + 0x4c) = 1;
  }
  iVar10 = FUN_00518460();
  if (((iVar10 != 0) && (bVar2 = FUN_0051eaf0(param_1), CONCAT31(extraout_var,bVar2) != 0)) &&
     (*(int **)(param_1 + 0xbc) != (int *)0x0)) {
    (**(code **)(**(int **)(param_1 + 0xbc) + 4))();
    FUN_0051eb40(param_1);
  }
  ExceptionList = local_c;
  return 1;
}

