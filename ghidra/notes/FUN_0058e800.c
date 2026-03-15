
int __thiscall FUN_0058e800(void *this,void *param_1)

{
  int iVar1;
  int *piVar2;
  
  iVar1 = FUN_00595530(param_1);
  if (iVar1 != 0) {
    FUN_005f50e0((int)this + 0x10);
    FUN_005f60f0(param_1);
    FUN_005f60f0(param_1);
    FUN_005f60f0(param_1);
    piVar2 = (int *)thunk_FUN_005f5060((int)this + 0x10);
    do {
      if (piVar2 == (int *)0x0) {
        return iVar1;
      }
      iVar1 = (**(code **)(*piVar2 + 0x10))(param_1);
      piVar2 = (int *)piVar2[4];
    } while (iVar1 != 0);
  }
  return iVar1;
}

