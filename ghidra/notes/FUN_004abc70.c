
int __thiscall FUN_004abc70(void *this,int param_1,int param_2,int param_3)

{
  void *this_00;
  HDC hDC;
  
  SetRect((LPRECT)((int)this + 0x60),0,param_1,param_2,param_3 + param_1);
  hDC = (HDC)0x0;
  *(undefined4 *)((int)this + 0x44) = 6;
  *(int *)((int)this + 0x48) = param_1;
  if (*(int *)((int)this + 0x40) != 0) {
    hDC = GetDC(*(HWND *)(*(int *)((int)this + 0x40) + 0x18));
  }
  if (*(void **)((int)this + 0x38) != (void *)0x0) {
    FUN_00601b80(*(void **)((int)this + 0x38),hDC);
    this_00 = *(void **)((int)this + 0x38);
    FUN_00601b30(this_00,*(int *)((int)this + 0x74) + 6,
                 ((uint)((*(int *)((int)this_00 + 0x18) - *(int *)((int)this_00 + 0x20)) + param_3)
                 >> 1) + param_1);
    FUN_00601b80(*(void **)((int)this + 0x38),hDC);
  }
  if (*(int *)((int)this + 0x40) != 0) {
    ReleaseDC(*(HWND *)(*(int *)((int)this + 0x40) + 0x18),hDC);
  }
  return param_3 + param_1;
}

