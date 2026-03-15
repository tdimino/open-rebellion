
int __thiscall FUN_005f5200(void *this,int param_1)

{
  int iVar1;
  void *this_00;
  int iVar2;
  
  iVar1 = *(int *)(param_1 + 4);
  this_00 = *(void **)(param_1 + 0xc);
  if (iVar1 == 0) {
    return param_1;
  }
  iVar2 = *(int *)(iVar1 + 8);
  if (iVar2 != 0) {
    *(int *)(iVar2 + 0xc) = param_1;
  }
  *(int *)(param_1 + 4) = iVar2;
  *(int *)(iVar1 + 8) = param_1;
  *(int *)(param_1 + 0xc) = iVar1;
  FUN_005ee440(param_1);
  if (this_00 != (void *)0x0) {
    FUN_005ee410(this_00,param_1,iVar1);
    *(void **)(iVar1 + 0xc) = this_00;
    FUN_005ee440(iVar1);
    return iVar1;
  }
  *(int *)((int)this + 4) = iVar1;
  *(undefined4 *)(iVar1 + 0xc) = 0;
  FUN_005ee440(iVar1);
  return iVar1;
}

