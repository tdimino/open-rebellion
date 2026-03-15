
void __thiscall FUN_005f6250(void *this,int param_1)

{
  int iVar1;
  void *pvVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  
  iVar5 = *(int *)(param_1 + 4);
  iVar1 = *(int *)(param_1 + 8);
  iVar4 = FUN_005f5c60(param_1);
  if (iVar4 == 0) {
    *(undefined4 *)((int)this + 8) = *(undefined4 *)(param_1 + 0x10);
  }
  else {
    *(undefined4 *)(iVar4 + 0x10) = *(undefined4 *)(param_1 + 0x10);
  }
  if (iVar5 == 0) {
    pvVar2 = *(void **)(param_1 + 0xc);
    if (iVar1 == 0) {
      if (pvVar2 != (void *)0x0) {
        FUN_005ee410(pvVar2,param_1,0);
      }
      iVar5 = 0;
    }
    else {
      *(void **)(iVar1 + 0xc) = pvVar2;
      iVar5 = iVar1;
      if (pvVar2 != (void *)0x0) {
        FUN_005ee410(pvVar2,param_1,iVar1);
      }
    }
  }
  else {
    pvVar2 = *(void **)(param_1 + 0xc);
    if (iVar1 == 0) {
      *(void **)(iVar5 + 0xc) = pvVar2;
      if (pvVar2 != (void *)0x0) {
        FUN_005ee410(pvVar2,param_1,iVar5);
      }
    }
    else {
      *(void **)(iVar1 + 0xc) = pvVar2;
      if (pvVar2 != (void *)0x0) {
        FUN_005ee410(pvVar2,param_1,iVar1);
      }
      iVar4 = iVar1;
      for (iVar3 = *(int *)(iVar1 + 4); iVar3 != 0; iVar3 = *(int *)(iVar3 + 4)) {
        iVar4 = iVar3;
      }
      *(int *)(iVar5 + 0xc) = iVar4;
      *(int *)(iVar4 + 4) = iVar5;
      iVar5 = iVar1;
    }
  }
  if (*(int *)((int)this + 4) == param_1) {
    *(int *)((int)this + 4) = iVar5;
  }
  FUN_005ee3f0(param_1);
  return;
}

