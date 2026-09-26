
int __thiscall FUN_005888f0(void *this,int *param_1,void *param_2,void *param_3,void *param_4)

{
  void *pvVar1;
  int *this_00;
  int iVar2;
  int iVar3;
  uint *puVar4;
  int **ppiVar5;
  int *local_14;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  pvVar1 = param_2;
  this_00 = param_1;
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_0064fa00;
  pvStack_c = ExceptionList;
  local_14 = (int *)0x0;
  local_10 = this;
  if (param_2 == (void *)0x0) {
    pvVar1 = *(void **)((int)this + 8);
    ppiVar5 = &local_14;
    ExceptionList = &pvStack_c;
    iVar2 = (**(code **)(*param_1 + 0x1bc))();
    iVar2 = FUN_00509330(pvVar1,iVar2,(int *)ppiVar5);
  }
  else {
    ppiVar5 = &local_14;
    ExceptionList = &pvStack_c;
    iVar2 = (**(code **)(*param_1 + 0x1bc))();
    iVar2 = FUN_004fd790(pvVar1,iVar2,(int *)ppiVar5);
  }
  FUN_004ece30(&param_1);
  iStack_4 = 0;
  if (local_14 == (int *)0x0) {
    iVar3 = 0;
    puVar4 = FUN_004025b0(this_00,(uint *)&param_2);
    iStack_4._0_1_ = 2;
    FUN_004f26d0(&param_1,puVar4);
  }
  else {
    iVar3 = (**(code **)(*local_14 + 0x1f0))();
    puVar4 = FUN_004025b0(local_14,(uint *)&param_2);
    iStack_4._0_1_ = 1;
    FUN_004f26d0(&param_1,puVar4);
  }
  iStack_4._0_1_ = 0;
  FUN_00619730();
  pvVar1 = param_3;
  iVar3 = FUN_005349e0(param_3,&param_1,iVar3,param_4);
  if ((iVar3 == 0) || (iVar2 == 0)) {
    iVar2 = 0;
  }
  else {
    iVar2 = 1;
  }
  if (((*(uint *)((int)pvVar1 + 0x78) & 4) == 0) && ((*(uint *)((int)pvVar1 + 0x78) & 0x10) != 0)) {
    puVar4 = FUN_0042d170(pvVar1,&param_2);
    iStack_4._0_1_ = 3;
    iVar3 = FUN_004ece60(puVar4);
    iStack_4 = (uint)iStack_4._1_3_ << 8;
    FUN_00619730();
    if (iVar3 != 0) {
      iVar3 = FUN_00534640(pvVar1,1,param_4);
      if ((iVar3 == 0) || (iVar2 == 0)) {
        iVar2 = 0;
      }
      else {
        iVar2 = 1;
      }
    }
  }
  if ((*(uint *)((int)pvVar1 + 0x78) & 0xc) != 0) {
    if ((*(uint *)((int)pvVar1 + 0x78) & 1) == 0) {
      *(int *)((int)local_10 + 0x2c) = *(int *)((int)local_10 + 0x2c) + -1;
    }
    else {
      *(int *)((int)local_10 + 0x30) = *(int *)((int)local_10 + 0x30) + -1;
    }
  }
  iStack_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return iVar2;
}

