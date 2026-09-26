
void __thiscall FUN_00609ed0(void *this,int param_1)

{
  int iVar1;
  bool bVar2;
  int iVar3;
  int iVar4;
  int local_8;
  int local_4;
  
  iVar3 = FUN_006098f0(this,param_1);
  if (iVar3 != 0) {
    bVar2 = false;
    param_1 = *(int *)(iVar3 + 0x40);
    iVar4 = *(int *)(iVar3 + 0x48);
    iVar1 = *(int *)((int)this + 0xf0);
    if ((iVar1 == 0) ||
       (iVar4 = *(int *)(iVar3 + 0x4c), param_1 = *(int *)(iVar3 + 0x44), iVar1 == 0)) {
      iVar3 = *(int *)((int)this + 0x38);
    }
    else {
      iVar3 = *(int *)((int)this + 0x3c);
    }
    if (iVar1 == 0) {
      FUN_0060a750((void *)((int)this + 0x94),&local_4,&local_8);
    }
    else {
      FUN_0060a6f0((void *)((int)this + 0x94),&local_4,&local_8);
    }
    if (param_1 < 0) {
      bVar2 = true;
      *(int *)((int)this + 0xc4) = *(int *)((int)this + 0xc4) + param_1;
    }
    else if (iVar3 < iVar4) {
      param_1 = iVar4 - iVar3;
      bVar2 = true;
      *(int *)((int)this + 0xc4) = *(int *)((int)this + 0xc4) + (iVar4 - iVar3);
    }
    if (bVar2) {
      if (*(int *)((int)this + 0xf0) != 0) {
        FUN_0060a6c0((void *)((int)this + 0x94),param_1);
      }
      FUN_00609e20(this);
      if (*(HWND *)((int)this + 0x18) != (HWND)0x0) {
        InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
      }
    }
  }
  return;
}

