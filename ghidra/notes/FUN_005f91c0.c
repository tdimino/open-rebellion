
int __thiscall FUN_005f91c0(void *this,int param_1,int param_2)

{
  int iVar1;
  uint uVar2;
  int *piVar3;
  uint uVar4;
  
  if (*(int *)((int)this + 0x108) == 0) {
    return 1;
  }
  if (*(uint *)((int)this + 0x94) < 2) {
    return 0xb;
  }
  if (param_1 == 0) {
    return 0x10;
  }
  iVar1 = FUN_005fb280(param_1);
  if (iVar1 == 0) {
    iVar1 = FUN_005f7ea0(this,0xf);
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
  if (*(int *)((int)this + 0x108) == 0) {
    return 1;
  }
  if (param_2 == 1) {
    uVar4 = 0;
    iVar1 = FUN_005f8a20((int)this);
    if (iVar1 != 0) {
      piVar3 = (int *)((int)this + 0x34);
      do {
        uVar2 = FUN_005f88c0((int)this);
        if (((uVar4 != uVar2) && (uVar4 < 0x10)) && (*piVar3 != -1)) {
          FUN_005fb3b0(*(void **)(param_1 + 0x1c),uVar4);
        }
        uVar4 = uVar4 + 1;
        piVar3 = piVar3 + 1;
        uVar2 = FUN_005f8a20((int)this);
      } while (uVar4 < uVar2);
    }
  }
  return 0;
}

