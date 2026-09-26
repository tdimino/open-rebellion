
bool __thiscall FUN_004160c0(void *this,void *param_1)

{
  int iVar1;
  void *this_00;
  BOOL BVar2;
  
  if (*(int **)((int)this + 0xc0) != (int *)0x0) {
    (**(code **)(**(int **)((int)this + 0xc0) + 8))();
    iVar1 = *(int *)((int)this + 0xc0);
    if (*(int *)((int)this + 0xb8) == iVar1) {
      *(undefined4 *)((int)this + 0xb8) = 0;
    }
    if (*(int *)((int)this + 0xbc) == iVar1) {
      *(undefined4 *)((int)this + 0xbc) = 0;
    }
    PostMessageA(*(HWND *)((int)this + 0x18),0x469,0,iVar1);
    *(undefined4 *)((int)this + 0xc0) = 0;
  }
  iVar1 = *(int *)((int)this + 0xb8);
  if (iVar1 != 0) {
    if (*(int *)((int)this + 0xc0) == iVar1) {
      *(undefined4 *)((int)this + 0xc0) = 0;
    }
    BVar2 = DestroyWindow(*(HWND *)(iVar1 + 0x18));
    if ((BVar2 != 0) && (*(undefined4 **)((int)this + 0xb8) != (undefined4 *)0x0)) {
      (**(code **)**(undefined4 **)((int)this + 0xb8))(1);
    }
    *(undefined4 *)((int)this + 0xb8) = 0;
  }
  iVar1 = *(int *)((int)this + 0xbc);
  if (iVar1 != 0) {
    if (*(int *)((int)this + 0xc0) == iVar1) {
      *(undefined4 *)((int)this + 0xc0) = 0;
    }
    DestroyWindow(*(HWND *)(iVar1 + 0x18));
    if (*(undefined4 **)((int)this + 0xbc) != (undefined4 *)0x0) {
      (**(code **)**(undefined4 **)((int)this + 0xbc))(1);
    }
    *(undefined4 *)((int)this + 0xbc) = 0;
  }
  FUN_00414330(this,9);
  this_00 = *(void **)((int)this + 0xbc);
  if (this_00 != (void *)0x0) {
    FUN_0059cb80(this_00,param_1);
  }
  return this_00 != (void *)0x0;
}

