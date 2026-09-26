
void __thiscall FUN_0045fe60(void *this,WPARAM param_1,LPARAM param_2)

{
  int iVar1;
  BOOL BVar2;
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
  case 0x21:
  case 0x28:
    SetFocus(*(HWND *)(*(int *)((int)this + 300) + 0x18));
    PostMessageA(*(HWND *)(*(int *)((int)this + 300) + 0x18),0x100,param_1,param_2);
    return;
  case 0x22:
    SetFocus(*(HWND *)(*(int *)((int)this + 300) + 0x18));
    PostMessageA(*(HWND *)(*(int *)((int)this + 300) + 0x18),0x100,param_1,param_2);
    return;
  case 0x25:
  case 0x27:
    if (*(int *)((int)this + 0x114) == 1) {
      iVar1 = *(int *)(*(int *)((int)this + 0x124) + 0x94);
      if (iVar1 == 0) {
        uVar3 = 0;
      }
      else {
        uVar3 = *(uint *)(iVar1 + 0x24);
      }
      iVar1 = FUN_00604500((void *)(*(int *)((int)this + 0x124) + 0x6c),uVar3);
      if (param_1 == 0x25) {
        iVar1 = FUN_005f5c60(iVar1);
      }
      else {
        iVar1 = *(int *)(iVar1 + 0x10);
      }
      if (iVar1 == 0) {
        iVar1 = FUN_005f5060(*(int *)((int)this + 0x124) + 0x6c);
        if (iVar1 != 0) {
          uVar3 = *(uint *)(iVar1 + 0x24);
LAB_0045ff12:
          FUN_0060d7e0(*(void **)((int)this + 0x124),uVar3,1);
        }
      }
      else {
        do {
          BVar2 = IsWindowVisible(*(HWND *)(iVar1 + 0x18));
          if (BVar2 != 0) {
            uVar3 = *(uint *)(iVar1 + 0x24);
            goto LAB_0045ff12;
          }
          if (param_1 == 0x25) {
            iVar1 = FUN_005f5c60(iVar1);
          }
          else {
            iVar1 = *(int *)(iVar1 + 0x10);
          }
        } while (iVar1 != 0);
      }
    }
    if (*(int *)((int)this + 0x114) == 2) {
      if (param_1 == 0x27) {
        (**(code **)(*(int *)this + 0x48))(0x84,0x84,0);
        return;
      }
      (**(code **)(*(int *)this + 0x48))(0x83,0x83,0);
      return;
    }
    break;
  case 0x26:
    SetFocus(*(HWND *)(*(int *)((int)this + 300) + 0x18));
    PostMessageA(*(HWND *)(*(int *)((int)this + 300) + 0x18),0x100,param_1,param_2);
    return;
  }
  return;
}

