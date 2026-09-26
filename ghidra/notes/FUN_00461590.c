
void __thiscall FUN_00461590(void *this,WPARAM param_1,LPARAM param_2)

{
  BOOL BVar1;
  int iVar2;
  uint uVar3;
  
  switch(param_1) {
  case 9:
    break;
  default:
    (**(code **)(**(int **)((int)this + 0x20) + 0x14))
              (*(int **)((int)this + 0x20),0x100,param_1,param_2);
    break;
  case 0x1b:
    (**(code **)(*(int *)this + 0x30))();
    return;
  case 0x25:
  case 0x27:
    iVar2 = *(int *)(*(int *)((int)this + 0x144) + 0x94);
    if (iVar2 == 0) {
      uVar3 = 0;
    }
    else {
      uVar3 = *(uint *)(iVar2 + 0x24);
    }
    iVar2 = FUN_00604500((void *)(*(int *)((int)this + 0x144) + 0x6c),uVar3);
    if (param_1 == 0x25) {
      iVar2 = FUN_005f5c60(iVar2);
    }
    else {
      iVar2 = *(int *)(iVar2 + 0x10);
    }
    if (iVar2 != 0) {
      do {
        BVar1 = IsWindowVisible(*(HWND *)(iVar2 + 0x18));
        if (BVar1 != 0) {
          FUN_0060d7e0(*(void **)((int)this + 0x144),*(uint *)(iVar2 + 0x24),1);
          return;
        }
        if (param_1 == 0x25) {
          iVar2 = FUN_005f5c60(iVar2);
        }
        else {
          iVar2 = *(int *)(iVar2 + 0x10);
        }
      } while (iVar2 != 0);
      return;
    }
    iVar2 = FUN_005f5060(*(int *)((int)this + 0x144) + 0x6c);
    if (iVar2 != 0) {
      FUN_0060d7e0(*(void **)((int)this + 0x144),*(uint *)(iVar2 + 0x24),1);
      return;
    }
    break;
  case 0x26:
    SetFocus(*(HWND *)(*(int *)((int)this + 0x148) + 0x18));
    return;
  case 0x28:
    iVar2 = (**(code **)(**(int **)(*(int *)((int)this + 0x14c) + 0xa0) + 8))();
    if (iVar2 != 0) {
      SetFocus(*(HWND *)(*(int *)((int)this + 0x14c) + 0x18));
      PostMessageA(*(HWND *)(*(int *)((int)this + 0x14c) + 0x18),0x100,param_1,param_2);
      return;
    }
  }
  return;
}

