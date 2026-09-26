
void __thiscall FUN_005dd140(void *this,int param_1)

{
  void *this_00;
  uint *puVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  
  puVar1 = (uint *)FUN_005ff440((int)this);
  piVar4 = (int *)((int)this + 0x9c);
  iVar3 = 5;
  do {
    if (*piVar4 != 0) {
      iVar2 = FUN_00602510(*piVar4);
      if (iVar2 != param_1) {
        FUN_006030f0((void *)*piVar4,4);
      }
    }
    piVar4 = piVar4 + 1;
    iVar3 = iVar3 + -1;
  } while (iVar3 != 0);
  if (param_1 == 0) {
    InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
  }
  else {
    if (4 < param_1 - 0xdd10U) {
      InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
      return;
    }
    if ((puVar1 != (uint *)0x0) &&
       (this_00 = *(void **)((int)this + (param_1 - 0xdd10U) * 4 + 0xb0), this_00 != (void *)0x0)) {
      FUN_005fcc30(this_00,puVar1,0xe,0xe,0,0,(undefined4 *)0x7c,0x6a);
      InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
      return;
    }
  }
  return;
}

