
uint __fastcall FUN_004a24b0(int *param_1)

{
  int iVar1;
  int *piVar2;
  void *this;
  int *piVar3;
  int iVar4;
  uint uVar5;
  undefined1 *puVar6;
  undefined1 local_34 [4];
  undefined1 local_30 [4];
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006378c0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar2 = FUN_004a25c0(param_1);
  uVar5 = 0;
  if (piVar2 == (int *)0x0) {
    ExceptionList = local_c;
    return 0;
  }
  if (param_1[0x71] != 0) {
    FUN_00536da0(local_2c,piVar2,3);
    local_4 = 0;
    FUN_00513120((int)local_2c);
    if (local_10 != 0) {
      do {
        piVar2 = FUN_0042d170((void *)param_1[0x71],local_30);
        puVar6 = local_34;
        local_4._0_1_ = 1;
        this = (void *)FUN_0052bed0((int)local_2c);
        piVar3 = FUN_0042d170(this,puVar6);
        iVar4 = *piVar3;
        iVar1 = *piVar2;
        FUN_00619730();
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
        if (iVar4 == iVar1) break;
        FUN_005130d0((int)local_2c);
      } while (local_10 != 0);
      if (local_10 != 0) {
        iVar4 = FUN_0052bed0((int)local_2c);
        uVar5 = *(uint *)(iVar4 + 0x24) >> 6 & 3;
      }
    }
    local_4 = 0xffffffff;
    FUN_00536ea0(local_2c);
  }
  ExceptionList = local_c;
  return uVar5;
}

