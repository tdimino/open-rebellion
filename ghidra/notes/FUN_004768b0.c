
void __thiscall FUN_004768b0(void *this,uint *param_1)

{
  uint *this_00;
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  
  this_00 = (uint *)((int)this + 0x34);
  uVar2 = *(uint *)((int)this + 0x34) >> 0x18;
  if ((uVar2 < 0x80) || (0x8f < uVar2)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    pvVar3 = FUN_004f5940((void *)(*(int *)((int)this + 0x58) + 0x44),this_00);
    if ((pvVar3 != (void *)0x0) && (*(int *)((int)pvVar3 + 0x38) == *(int *)((int)this + 0x18))) {
      FUN_004317d0(pvVar3,0);
    }
    FUN_004ece80(this_00);
  }
  if ((*param_1 >> 0x18 < 0x80) || (0x8f < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    pvVar3 = FUN_004f5940((void *)(*(int *)((int)this + 0x58) + 0x44),param_1);
    if ((pvVar3 != (void *)0x0) && (*(int *)((int)pvVar3 + 0x38) == 0)) {
      FUN_004317d0(pvVar3,*(undefined4 *)((int)this + 0x18));
      FUN_004f26d0(this_00,param_1);
      FUN_004769f0(this,*(int *)((int)pvVar3 + 0x3c));
    }
  }
  if ((*this_00 >> 0x18 < 0x80) || (0x8f < *this_00 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (!bVar1) {
    FUN_004769f0(this,0);
  }
  return;
}

