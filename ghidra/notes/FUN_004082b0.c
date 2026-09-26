
void __thiscall FUN_004082b0(void *this,void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  void *this_00;
  uint uVar2;
  void *pvVar3;
  
  pvVar3 = param_1;
  param_1 = (void *)FUN_00604500((void *)((int)this + 0x6c),(uint)param_1);
  if ((*(int *)((int)this + 0xa8) < 0x14) || (0x17 < *(int *)((int)this + 0xa8))) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  switch(pvVar3) {
  case (void *)0x99:
    pvVar3 = *(void **)((int)this + 0xe8);
    this_00 = *(void **)((int)this + 0x100);
    uVar2 = 0x100;
    if (bVar1) {
      FUN_00596b70();
    }
    else {
      FUN_00401ab0(~*(uint *)((int)this + 0xa0) & 0x100);
    }
    break;
  default:
    uVar2 = 0;
    pvVar3 = (void *)0x0;
    this_00 = (void *)0x0;
    break;
  case (void *)0xa1:
    pvVar3 = *(void **)((int)this + 0xec);
    this_00 = *(void **)((int)this + 0x104);
    uVar2 = 0x10000000;
    break;
  case (void *)0xa2:
    pvVar3 = *(void **)((int)this + 0xf0);
    this_00 = *(void **)((int)this + 0x108);
    uVar2 = 0x1000000;
    break;
  case (void *)0xa3:
    pvVar3 = *(void **)((int)this + 0xf4);
    this_00 = *(void **)((int)this + 0x10c);
    uVar2 = 0x100000;
    break;
  case (void *)0xa4:
    pvVar3 = *(void **)((int)this + 0xf8);
    this_00 = *(void **)((int)this + 0x110);
    uVar2 = 0x10000;
    break;
  case (void *)0xa6:
    pvVar3 = *(void **)((int)this + 0xfc);
    this_00 = *(void **)((int)this + 0x114);
    uVar2 = 0x2000000;
  }
  if (param_1 != (void *)0x0) {
    bVar1 = FUN_00603120(param_1,4);
    if (CONCAT31(extraout_var,bVar1) == 0) {
      *(uint *)((int)this + 0xa0) = *(uint *)((int)this + 0xa0) & ~uVar2;
      if (pvVar3 != (void *)0x0) {
        FUN_00601c90(pvVar3,0x2008000,(HWND)0x0);
        FUN_00601dc0(pvVar3,*(HWND *)((int)this + 0x18),1);
      }
      if (this_00 == (void *)0x0) {
        return;
      }
      param_1 = (void *)CONCAT22(DAT_0065d420,0x1032);
      FUN_00601c90(this_00,0x2008000,(HWND)0x0);
    }
    else {
      *(uint *)((int)this + 0xa0) = *(uint *)((int)this + 0xa0) | uVar2;
      if (pvVar3 != (void *)0x0) {
        FUN_00601c90(pvVar3,0x200ff00,(HWND)0x0);
        FUN_00601dc0(pvVar3,*(HWND *)((int)this + 0x18),1);
      }
      if (this_00 == (void *)0x0) {
        return;
      }
      param_1 = (void *)CONCAT22(DAT_0065d420,0x1031);
      FUN_00601c90(this_00,0x200ff00,(HWND)0x0);
    }
    FUN_006019a0(this_00,&param_1);
    FUN_00601dc0(this_00,*(HWND *)((int)this + 0x18),1);
  }
  return;
}

