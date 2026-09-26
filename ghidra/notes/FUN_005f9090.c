
int __thiscall FUN_005f9090(void *this,int param_1,int param_2,int param_3)

{
  int iVar1;
  
  if (*(int *)((int)this + 0x108) == 0) {
    return 1;
  }
  if (1 < *(uint *)((int)this + 0x94)) {
    iVar1 = FUN_005f8ff0(this,param_2);
    if ((iVar1 != 0) && (iVar1 != -1)) {
      if ((iVar1 == *(int *)((int)this + 0x110)) || (param_2 == *(int *)((int)this + 0x10c))) {
        return 0xd;
      }
      if (param_1 == 0) {
        return 0x10;
      }
      if (param_3 == 1) {
        *(undefined1 *)(param_1 + 0x26) = 1;
      }
      iVar1 = FUN_005fb280(param_1);
      if (iVar1 == 0) {
        iVar1 = FUN_005f7ea0(this,param_2);
        if (*(int *)((int)this + 0x108) == 0) {
          return 1;
        }
        FUN_005ae1e0();
        if (iVar1 != 0) {
          return iVar1;
        }
        iVar1 = FUN_005fb280(param_1);
        if (iVar1 == 0) {
          FUN_005ae1e0();
          return 0xc;
        }
      }
      FUN_005fb3b0(*(void **)(param_1 + 0x1c),param_2);
      return 0;
    }
  }
  return 0xb;
}

