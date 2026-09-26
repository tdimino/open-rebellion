
undefined4 __thiscall FUN_0060c810(void *this,undefined4 param_1)

{
  undefined4 uVar1;
  int iVar2;
  
  uVar1 = 0;
  switch(param_1) {
  case 0xd:
    if (*(int *)((int)this + 0xe8) != 0) {
      FUN_0060cbf0((int)this);
      return 1;
    }
    break;
  case 0x1b:
    if (*(int *)((int)this + 0xe8) != 0) {
      FUN_0060cbf0((int)this);
      uVar1 = 1;
    }
    break;
  case 0x26:
    if (*(int *)((int)this + 0xe8) == 0) {
      PostMessageA(*(HWND *)((int)this + 0x18),0x111,0x3ea,0);
      return 1;
    }
    if ((*(int **)((int)this + 0xe0) == (int *)0x0) ||
       (iVar2 = (**(code **)(**(int **)((int)this + 0xe0) + 0x10))(), iVar2 == 0)) {
      uVar1 = (**(code **)(*(int *)((int)this + 0xd0) + 8))();
    }
    else {
      uVar1 = (**(code **)(**(int **)((int)this + 0xe0) + 0x10))();
    }
    goto LAB_0060c8d5;
  case 0x28:
    if (*(int *)((int)this + 0xe8) == 0) {
      PostMessageA(*(HWND *)((int)this + 0x18),0x111,0x3ea,0);
      return 1;
    }
    if ((*(int **)((int)this + 0xe0) == (int *)0x0) ||
       (iVar2 = (**(code **)(**(int **)((int)this + 0xe0) + 0xc))(), iVar2 == 0)) goto LAB_0060c8db;
    uVar1 = (**(code **)(**(int **)((int)this + 0xe0) + 0xc))();
LAB_0060c8d5:
    *(undefined4 *)((int)this + 0xe0) = uVar1;
LAB_0060c8db:
    FUN_0060cd50(*(int *)((int)this + 0xe8));
    return 1;
  }
  return uVar1;
}

