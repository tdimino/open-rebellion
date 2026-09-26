
void * __thiscall FUN_005aded0(void *this,int param_1)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  undefined4 uVar4;
  void *this_00;
  int *piVar5;
  void *pvVar6;
  void *unaff_EDI;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652edb;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005ade20(this,param_1);
  this_00 = (void *)FUN_00618b70(0xbc);
  pvVar6 = (void *)0x0;
  local_4 = 0;
  if (this_00 != (void *)0x0) {
    pvVar6 = FUN_005cbf50(this_00,*(uint *)(param_1 + 0xec));
  }
  local_4 = 0xffffffff;
  piVar5 = (int *)FUN_005aaf90();
  (**(code **)(*piVar5 + 0x4c))(pvVar6);
  iVar1 = *(int *)(param_1 + 0x168);
  uVar2 = *(undefined4 *)((int)pvVar6 + 0x18);
  iVar3 = *(int *)((int)this + 0x9ec);
  uVar4 = *(undefined4 *)(param_1 + 0xec);
  piVar5 = (int *)FUN_005aaf90();
  (**(code **)(*piVar5 + 0x28))(0x30,uVar2,uVar4,iVar1 == iVar3);
  *(undefined4 *)(param_1 + 0x178) = *(undefined4 *)((int)pvVar6 + 0x18);
  ExceptionList = unaff_EDI;
  return pvVar6;
}

