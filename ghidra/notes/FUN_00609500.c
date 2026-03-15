
void __thiscall FUN_00609500(void *this,int param_1,int param_2)

{
  int *piVar1;
  
  if ((*(uint *)((int)this + 0xf4) & 0x10000) != 0) {
    FUN_006094b0(this);
  }
  piVar1 = (int *)(**(code **)(**(int **)((int)this + 0xa0) + 8))();
  while (piVar1 != (int *)0x0) {
    if (piVar1[3] == param_1) {
      piVar1[0xf] = piVar1[0xf] | 1;
      FUN_00609a00(this);
      piVar1 = (int *)0x0;
    }
    else {
      piVar1 = (int *)(**(code **)(*piVar1 + 0xc))();
    }
  }
  if (param_2 != 0) {
    PostMessageA(*(HWND *)(*(int *)((int)this + 0x20) + 0x18),0x111,
                 *(uint *)((int)this + 0x24) & 0xffff | 0x29b0000,0);
  }
  return;
}

