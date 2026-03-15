
int __thiscall FUN_004fc710(void *this,void *param_1)

{
  int *piVar1;
  bool bVar2;
  void **ppvVar3;
  uint uVar4;
  void *this_00;
  int iVar5;
  int iVar6;
  undefined4 local_2c [7];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006401f0;
  bVar2 = false;
  iVar6 = 1;
  ppvVar3 = &pvStack_c;
  pvStack_c = ExceptionList;
  for (piVar1 = *(int **)((int)this + 0x1c); ExceptionList = ppvVar3, piVar1 != (int *)0x0;
      piVar1 = (int *)piVar1[7]) {
    if (bVar2) goto LAB_004fc794;
    local_4 = 0;
    uVar4 = (**(code **)(*piVar1 + 4))();
    if ((uVar4 < 0xf2) || (0xf2 < uVar4)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    ppvVar3 = (void **)ExceptionList;
  }
  if (bVar2) {
LAB_004fc794:
    FUN_004fcd00(local_2c,this,4);
    local_4 = 1;
    FUN_00513120((int)local_2c);
    while (iStack_10 != 0) {
      this_00 = (void *)FUN_0052bed0((int)local_2c);
      FUN_005130d0((int)local_2c);
      iVar5 = FUN_004fc710(this_00,param_1);
      if ((iVar5 == 0) || (iVar6 == 0)) {
        iVar6 = 0;
      }
      else {
        iVar6 = 1;
      }
    }
    uVar4 = FUN_004fc840(this,param_1);
    if ((uVar4 == 0) || (iVar6 == 0)) {
      iVar6 = 0;
    }
    else {
      iVar6 = 1;
    }
    local_4 = 0xffffffff;
    FUN_004fcd80(local_2c);
  }
  ExceptionList = pvStack_c;
  return iVar6;
}

