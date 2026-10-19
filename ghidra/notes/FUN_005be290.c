
void __thiscall FUN_005be290(int param_1,int *param_2,int param_3)

{
  int *piVar1;
  int iVar2;
  int iVar3;
  void *this;
  
  if (param_3 == 1) {
    this = (void *)(param_1 + 0x908 + *(int *)(param_1 + 0x9ec) * 0xc);
    piVar1 = (int *)thunk_FUN_005f5060((int)this);
    if (piVar1 != (int *)0x0) {
      do {
        iVar2 = (**(code **)(*piVar1 + 0x1c))();
        iVar3 = (**(code **)(*param_2 + 0x1c))();
        if (iVar2 == iVar3) {
          return;
        }
        piVar1 = (int *)(**(code **)(*piVar1 + 8))();
      } while (piVar1 != (int *)0x0);
      FUN_005f5440(this,param_2);
      return;
    }
  }
  else {
    this = (void *)(param_1 + 0x908 + (uint)(*(int *)(param_1 + 0x9ec) == 0) * 0xc);
    for (piVar1 = (int *)thunk_FUN_005f5060((int)this); piVar1 != (int *)0x0;
        piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
      iVar2 = (**(code **)(*piVar1 + 0x1c))();
      iVar3 = (**(code **)(*param_2 + 0x1c))();
      if (iVar2 == iVar3) {
        return;
      }
    }
  }
  FUN_005f5440(this,param_2);
  return;
}

