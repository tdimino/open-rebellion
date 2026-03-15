
undefined4 __thiscall FUN_00588b90(void *this,int *param_1,int *param_2,void *param_3,void *param_4)

{
  void *pvVar1;
  uint uVar2;
  bool bVar3;
  int *this_00;
  void *pvVar4;
  int iVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int **ppiVar10;
  int *piVar11;
  int *local_10;
  int iStack_c;
  int iStack_8;
  void *local_4;
  
  pvVar1 = param_3;
  this_00 = param_2;
  local_10 = (int *)0x0;
  local_4 = this;
  if (param_3 == (void *)0x0) {
    pvVar1 = *(void **)((int)this + 8);
    ppiVar10 = &local_10;
    iVar5 = (**(code **)(*param_2 + 0x1bc))();
    iVar5 = FUN_00509330(pvVar1,iVar5,(int *)ppiVar10);
  }
  else {
    ppiVar10 = &local_10;
    iVar5 = (**(code **)(*param_2 + 0x1bc))();
    iVar5 = FUN_004fd790(pvVar1,iVar5,(int *)ppiVar10);
  }
  iStack_c = 0;
  if (local_10 != (int *)0x0) {
    iStack_c = (**(code **)(*local_10 + 0x1e0))();
  }
  param_2 = (int *)0x0;
  iStack_8 = 0;
  piVar11 = &iStack_8;
  uVar2 = param_1[9];
  ppiVar10 = &param_2;
  uVar6 = (uint)(param_3 != (void *)0x0);
  iVar7 = (**(code **)(*this_00 + 0x1c4))();
  iVar9 = iStack_c;
  iVar8 = (**(code **)(*param_1 + 0x1e0))();
  iVar9 = FUN_0055e410(uVar2 >> 6 & 3,iVar8,iVar9,iVar7,uVar6,ppiVar10,piVar11);
  pvVar1 = local_4;
  if ((iVar9 == 0) || (iVar5 == 0)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  FUN_005897b0();
  pvVar4 = param_4;
  if (param_2 != (int *)0x0) {
    iVar5 = FUN_00534720(param_1,1,param_4);
    if ((iVar5 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    iVar5 = FUN_00558070(this_00,1,pvVar4);
    if ((iVar5 != 0) && (bVar3)) {
      *(int *)((int)local_4 + 0x34) = *(int *)((int)local_4 + 0x34) + -1;
      return 1;
    }
    *(int *)((int)local_4 + 0x34) = *(int *)((int)local_4 + 0x34) + -1;
    return 0;
  }
  iVar5 = FUN_005888f0(pvVar1,this_00,param_3,param_1,param_4);
  if ((iVar5 != 0) && (bVar3)) {
    return 1;
  }
  return 0;
}

