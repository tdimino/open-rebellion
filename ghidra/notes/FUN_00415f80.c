
bool __thiscall FUN_00415f80(void *this,undefined4 param_1)

{
  int iVar1;
  int *piVar2;
  BOOL BVar3;
  
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
    BVar3 = DestroyWindow(*(HWND *)(iVar1 + 0x18));
    if ((BVar3 != 0) && (*(undefined4 **)((int)this + 0xb8) != (undefined4 *)0x0)) {
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
  *(undefined4 *)((int)this + 0xfc) = 1;
  *(undefined4 *)((int)this + 0x100) = 0;
  *(undefined4 *)((int)this + 0xf8) = 0;
  FUN_00414330(this,8);
  piVar2 = *(int **)((int)this + 0xb8);
  if (piVar2 != (int *)0x0) {
    (**(code **)(*piVar2 + 0x40))(param_1);
    FUN_0041d310(*(undefined4 *)((int)this + 0x108));
  }
  return piVar2 != (int *)0x0;
}

