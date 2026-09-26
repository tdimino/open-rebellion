
int __thiscall FUN_005326f0(void *this,undefined4 param_1,undefined4 param_2,void *param_3)

{
  uint *puVar1;
  int iVar2;
  int iVar3;
  uint *puVar4;
  int *piVar5;
  int *piVar6;
  int local_1c;
  uint local_18;
  uint local_14;
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00645688;
  local_c = ExceptionList;
  iVar3 = *(int *)((int)this + 0x9c);
  ExceptionList = &local_c;
  puVar1 = FUN_004025b0(this,&local_18);
  piVar6 = &local_1c;
  piVar5 = &local_10;
  puVar4 = &local_14;
  local_4 = 0;
  iVar2 = FUN_00530a20(puVar1);
  iVar3 = FUN_0051cc30(iVar2,iVar3,puVar4,piVar5,piVar6);
  local_4 = 0xffffffff;
  FUN_00619730();
  if ((iVar3 != 0) && (local_1c = FUN_0055bf80(local_1c,*(int *)((int)this + 0xc0)), iVar3 != 0)) {
    if (local_14 == 0) {
      iVar3 = FUN_0052f440(this,1,param_3);
    }
    else if (local_1c <= *(int *)((int)this + 0x90)) {
      iVar2 = FUN_0052f280(this,local_10,param_3);
      iVar3 = 0;
      if (iVar2 != 0) {
        iVar3 = FUN_0052f130(this,*(int *)((int)this + 0x90) - local_1c,param_3);
      }
    }
  }
  ExceptionList = local_c;
  return iVar3;
}

