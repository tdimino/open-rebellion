
void * __thiscall FUN_005a34e0(void *this,int *param_1,int *param_2,int *param_3,uint param_4)

{
  void *pvVar1;
  int *piVar2;
  int *piVar3;
  int iVar4;
  undefined4 uVar5;
  uint uVar6;
  int *piVar7;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652358;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_0059e880(this);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0066bc48;
  FUN_005c0e70();
  (**(code **)(*param_1 + 0x10))();
  pvVar1 = (void *)FUN_005aaf90();
  piVar2 = FUN_005ade70(pvVar1,param_1);
  iVar4 = piVar2[6];
  uVar6 = param_4;
  pvVar1 = (void *)FUN_0059ed70();
  FUN_005a03f0(pvVar1,uVar6,iVar4);
  for (piVar3 = (int *)(**(code **)(*param_3 + 0x10))(); piVar3 != (int *)0x0;
      piVar3 = (int *)(**(code **)(*piVar3 + 8))()) {
    piVar7 = piVar2;
    iVar4 = (**(code **)(*piVar3 + 0x10))();
    pvVar1 = (void *)FUN_005aaf90();
    FUN_005a8e30(pvVar1,iVar4,piVar7);
  }
  for (piVar3 = (int *)(**(code **)(*param_2 + 0xc))(); piVar3 != (int *)0x0;
      piVar3 = (int *)(**(code **)(*piVar3 + 0xc))()) {
    piVar7 = piVar2;
    iVar4 = (**(code **)(*piVar3 + 8))();
    pvVar1 = (void *)FUN_005aaf90();
    FUN_005a8f20(pvVar1,iVar4,piVar7);
  }
  uVar6 = param_4;
  pvVar1 = (void *)FUN_0059ed70();
  FUN_0059fc50(pvVar1,uVar6);
  *(uint *)((int)this + 0x54) = param_4;
  *(int **)((int)this + 0x5c) = piVar2;
  *(int *)((int)this + 0x58) = piVar2[6];
  iVar4 = FUN_0059ed70();
  FUN_0059ef60(iVar4);
  uVar5 = FUN_005c6070(piVar2 + 0x11);
  pvVar1 = (void *)FUN_0059ed70();
  FUN_0051c9b0(pvVar1,uVar5);
  iVar4 = FUN_005c6070(param_3);
  if ((iVar4 != 0) && (*(int *)(iVar4 + 0x168) == *(int *)(*(int *)((int)this + 0x5c) + 0x50))) {
    FUN_005a3630(this,1);
  }
  ExceptionList = pvStack_c;
  return this;
}

