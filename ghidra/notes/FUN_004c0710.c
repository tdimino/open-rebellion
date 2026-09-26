
void * __thiscall FUN_004c0710(void *this,undefined4 param_1,undefined4 param_2)

{
  undefined4 uVar1;
  uint uVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063a358;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00439320(this,param_1,param_2);
  *(undefined ***)this = &PTR_FUN_0065c4a0;
  *(undefined4 *)((int)this + 0x164) = 0x56;
  local_4 = 0;
  DAT_006b28b0 = DAT_006b28b0 | 0x1000;
  uVar1 = FUN_00618b70(*(int *)((int)this + 0x164) << 2);
  *(undefined4 *)((int)this + 0x168) = uVar1;
  uVar1 = FUN_00618b70(*(int *)((int)this + 0x164) << 2);
  *(undefined4 *)((int)this + 0x16c) = uVar1;
  uVar2 = 0;
  if (*(int *)((int)this + 0x164) != 0) {
    do {
      uVar2 = uVar2 + 1;
      *(undefined4 *)(*(int *)((int)this + 0x168) + -4 + uVar2 * 4) = 0;
      *(undefined4 *)(*(int *)((int)this + 0x16c) + -4 + uVar2 * 4) = 0;
    } while (uVar2 < *(uint *)((int)this + 0x164));
  }
  *(undefined4 *)(*(int *)((int)this + 0x168) + 0x128) = 40000;
  ExceptionList = local_c;
  return this;
}

