
void __thiscall FUN_004710f0(void *this,int param_1,undefined4 param_2)

{
  int iVar1;
  BOOL BVar2;
  uint uVar3;
  
  if (param_1 == 0x1b) {
    (**(code **)(*(int *)this + 0x30))();
    FUN_0041ce80(*(int **)((int)this + 0x174));
    FUN_00401240();
  }
  else {
    if ((param_1 != 0x25) && (param_1 != 0x27)) {
      (**(code **)(**(int **)((int)this + 0x20) + 0x14))
                (*(int **)((int)this + 0x20),0x100,param_1,param_2);
      return;
    }
    iVar1 = *(int *)(*(int *)((int)this + 0x160) + 0x94);
    if (iVar1 == 0) {
      uVar3 = 0;
    }
    else {
      uVar3 = *(uint *)(iVar1 + 0x24);
    }
    iVar1 = FUN_00604500((void *)(*(int *)((int)this + 0x160) + 0x6c),uVar3);
    if (param_1 == 0x25) {
      iVar1 = FUN_005f5c60(iVar1);
    }
    else {
      iVar1 = *(int *)(iVar1 + 0x10);
    }
    if (iVar1 != 0) {
      do {
        BVar2 = IsWindowVisible(*(HWND *)(iVar1 + 0x18));
        if (BVar2 != 0) {
          FUN_0060d7e0(*(void **)((int)this + 0x160),*(uint *)(iVar1 + 0x24),1);
          return;
        }
        if (param_1 == 0x25) {
          iVar1 = FUN_005f5c60(iVar1);
        }
        else {
          iVar1 = *(int *)(iVar1 + 0x10);
        }
      } while (iVar1 != 0);
      return;
    }
    iVar1 = FUN_005f5060(*(int *)((int)this + 0x160) + 0x6c);
    if (iVar1 != 0) {
      FUN_0060d7e0(*(void **)((int)this + 0x160),*(uint *)(iVar1 + 0x24),1);
      return;
    }
  }
  return;
}

