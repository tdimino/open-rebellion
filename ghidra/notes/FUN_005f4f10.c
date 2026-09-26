
void __thiscall FUN_005f4f10(void *this,int param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  
  FUN_005ee3f0(param_1);
  iVar2 = 0;
  iVar3 = *(int *)((int)this + 4);
  if (*(int *)((int)this + 4) == 0) {
    *(int *)((int)this + 4) = param_1;
    *(undefined4 *)(param_1 + 4) = 0;
    *(undefined4 *)(param_1 + 8) = 0;
    *(undefined4 *)(param_1 + 0xc) = 0;
    *(undefined4 *)(param_1 + 0x10) = 0;
    FUN_005f5100(this,param_1);
    return;
  }
  do {
    iVar1 = (**(code **)(*(int *)this + 8))(param_1,iVar3);
    if (iVar1 == 0) {
      if (param_1 == iVar3) {
        return;
      }
      iVar1 = *(int *)(iVar3 + 8);
      if (iVar1 == 0) {
        *(int *)(iVar3 + 8) = param_1;
        *(int *)(param_1 + 0xc) = iVar3;
        if (iVar3 != 0) {
          *(int *)(iVar3 + 0x10) = param_1;
        }
      }
    }
    else {
      iVar1 = *(int *)(iVar3 + 4);
      if (iVar1 == 0) {
        *(int *)(iVar3 + 4) = param_1;
        *(int *)(param_1 + 0xc) = iVar3;
        if (iVar2 != 0) {
          *(int *)(iVar2 + 0x10) = param_1;
        }
      }
      *(int *)(param_1 + 0x10) = iVar3;
      iVar3 = iVar2;
    }
    iVar2 = iVar3;
    iVar3 = iVar1;
  } while (iVar1 != 0);
  FUN_005f5100(this,param_1);
  return;
}

