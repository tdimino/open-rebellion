
undefined4 __thiscall FUN_00522a90(void *this,uint *param_1)

{
  uint uVar1;
  undefined4 local_3c [11];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006439f8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = 1;
  FUN_00525bb0(local_3c,this);
  local_4 = 0;
  FUN_00525930((int)local_3c);
  uVar1 = *param_1;
  while ((uVar1 != 0 && (local_10 != 0))) {
    *param_1 = ~*(uint *)(local_10 + 0x50) >> 0xb & 1;
    FUN_005258f0((int)local_3c);
    uVar1 = *param_1;
  }
  local_4 = 0xffffffff;
  *(uint *)(*(int *)((int)this + 0x54) + 0x18) = *param_1;
  FUN_00525c50(local_3c);
  ExceptionList = local_c;
  return 1;
}

