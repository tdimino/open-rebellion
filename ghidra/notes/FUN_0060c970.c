
undefined4 __thiscall FUN_0060c970(void *this,int param_1)

{
  int iVar1;
  
  if (((*(int *)((int)this + 0xe0) != 0) && (param_1 != 0)) &&
     (*(int *)(*(int *)((int)this + 0xe0) + 0xc) != param_1)) {
    iVar1 = (**(code **)(*(int *)((int)this + 0xd0) + 8))();
    *(int *)((int)this + 0xe0) = iVar1;
    while ((iVar1 != 0 && ((*(int **)((int)this + 0xe0))[3] != param_1))) {
      iVar1 = (**(code **)(**(int **)((int)this + 0xe0) + 0xc))();
      *(int *)((int)this + 0xe0) = iVar1;
    }
    InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
    if (*(int *)((int)this + 0x20) != 0) {
      PostMessageA(*(HWND *)(*(int *)((int)this + 0x20) + 0x18),0x111,0x29b,0);
    }
  }
  FUN_0060cbf0((int)this);
  return *(undefined4 *)((int)this + 0xe0);
}

