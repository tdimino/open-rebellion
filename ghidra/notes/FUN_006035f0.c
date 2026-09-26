
void __thiscall FUN_006035f0(void *this,uint param_1)

{
  uint uVar1;
  void *pvVar2;
  bool bVar3;
  undefined3 extraout_var;
  void *this_00;
  
  uVar1 = *(uint *)((int)this + 0xc4);
  this_00 = *(void **)((int)this + 0x94);
  if ((uVar1 & 2 | 0x40) == 0) {
    if ((uVar1 & 4) == 0) goto LAB_0060362b;
    pvVar2 = *(void **)((int)this + 0xa0);
  }
  else {
    pvVar2 = *(void **)((int)this + 0x9c);
  }
  if (pvVar2 != (void *)0x0) {
    this_00 = pvVar2;
  }
LAB_0060362b:
  if (((uVar1 & 2) != 0) || (this_00 == (void *)0x0)) {
    *(undefined4 *)((int)this + 0xf8) = 0;
    return;
  }
  if ((uVar1 & 0x10) != 0) {
    bVar3 = FUN_005fca00(this_00,param_1 & 0xffff,param_1 >> 0x10);
    *(uint *)((int)this + 0xf8) = CONCAT31(extraout_var,bVar3);
    return;
  }
  if (((int)(param_1 & 0xffff) <= *(int *)((int)this + 0x38)) &&
     ((int)(param_1 >> 0x10) <= *(int *)((int)this + 0x3c))) {
    *(undefined4 *)((int)this + 0xf8) = 1;
    return;
  }
  *(undefined4 *)((int)this + 0xf8) = 0;
  return;
}

