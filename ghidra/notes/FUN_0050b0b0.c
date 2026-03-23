
int __thiscall FUN_0050b0b0(void *this,void *param_1)

{
  int *piVar1;
  int iVar2;
  void *pvVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  undefined4 auStack_6c [7];
  int iStack_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 auStack_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641b08;
  pvStack_c = ExceptionList;
  iVar5 = 0;
  iVar6 = 1;
  ExceptionList = &pvStack_c;
  FUN_0052d690(local_4c,this,3,*(uint *)((int)this + 0x24) >> 6 & 3);
  local_4 = 0;
  FUN_00513120((int)local_4c);
  while (local_30 != 0) {
    piVar1 = (int *)FUN_0052bed0((int)local_4c);
    iVar2 = (**(code **)(*piVar1 + 0x1c8))();
    iVar5 = iVar5 + iVar2;
    FUN_005130d0((int)local_4c);
  }
  if (*(int *)((int)this + 100) < iVar5) {
    pvVar3 = FUN_0052d610(auStack_2c,this,3);
    local_4._0_1_ = 1;
    iVar2 = FUN_00513180((int)pvVar3);
    local_4._0_1_ = 0;
    FUN_0052d710(auStack_2c);
    iVar7 = 0;
    iVar4 = FUN_00509660((int)this);
    iVar2 = FUN_0053e130(iVar2 - iVar4,iVar7);
    FUN_0052d610(auStack_6c,this,3);
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_00513120((int)auStack_6c);
    for (; ((iStack_50 != 0 && (iVar6 != 0)) && (iVar2 != 0)); iVar2 = iVar2 + -1) {
      piVar1 = (int *)FUN_0052bed0((int)auStack_6c);
      iVar6 = (**(code **)(*piVar1 + 0xb8))(param_1);
      FUN_005130d0((int)auStack_6c);
    }
    local_4 = local_4 & 0xffffff00;
    FUN_0052d710(auStack_6c);
  }
  if (iVar6 != 0) {
    iVar6 = FUN_0053e140(iVar5,*(int *)((int)this + 100));
    iVar6 = FUN_00509e40(this,iVar6,param_1);
  }
  local_4 = 0xffffffff;
  FUN_0052d710(local_4c);
  ExceptionList = pvStack_c;
  return iVar6;
}

