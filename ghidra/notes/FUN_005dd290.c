
void __thiscall FUN_005dd290(void *this,int *param_1)

{
  int iVar1;
  int iVar2;
  void *this_00;
  int iVar3;
  int *piVar4;
  bool bVar5;
  
  piVar4 = param_1;
  FUN_005ef320((int)this);
  iVar3 = 0;
  bVar5 = param_1 == (int *)0x0;
  *(int **)((int)this + 200) = param_1;
  param_1 = (int *)0x0;
  if (bVar5) {
    return;
  }
  iVar1 = (**(code **)(*piVar4 + 4))();
  if (iVar1 == 1) {
    if (*(int *)((int)this + 200) == 0) {
      iVar1 = 0;
    }
    else {
      iVar1 = *(int *)((int)this + 200) + -0xd4;
    }
    iVar3 = FUN_00596af0(iVar1);
    param_1 = (int *)FUN_00596b30(iVar1);
  }
  else {
    iVar1 = (**(code **)(**(int **)((int)this + 200) + 4))();
    if ((iVar1 == 3) || (iVar1 = (**(code **)(**(int **)((int)this + 200) + 4))(), iVar1 == 4)) {
      iVar3 = *(int *)((int)this + 200);
      param_1 = (int *)FUN_00596b60(iVar3);
      iVar3 = FUN_00596b20(iVar3);
    }
  }
  if (iVar3 != 0) {
    piVar4 = (int *)((int)this + 0x9c);
    iVar1 = 5;
    do {
      if ((*piVar4 != 0) && (iVar2 = FUN_00602510(*piVar4), iVar2 == iVar3 + 0xdd10)) {
        FUN_006030c0((void *)*piVar4,4);
        FUN_005dd140(this,iVar3 + 0xdd10);
      }
      piVar4 = piVar4 + 1;
      iVar1 = iVar1 + -1;
    } while (iVar1 != 0);
  }
  this_00 = (void *)FUN_00604500((void *)((int)this + 0x6c),0xdd20);
  if (this_00 != (void *)0x0) {
    if (param_1 == (int *)0x1) {
      FUN_006030c0(this_00,4);
      return;
    }
    FUN_006030f0(this_00,4);
  }
  return;
}

