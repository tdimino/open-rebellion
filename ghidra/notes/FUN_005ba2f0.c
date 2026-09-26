
void __fastcall FUN_005ba2f0(int param_1)

{
  void *this;
  int *this_00;
  uint uVar1;
  
  uVar1 = *(uint *)(param_1 + 0x164);
  if (uVar1 != 0) {
    this = (void *)FUN_00596bd0();
    this_00 = FUN_005a6360(this,uVar1);
    FUN_005b8400(this_00,param_1);
    *(undefined4 *)(param_1 + 0x35c) = 0;
    *(undefined4 *)(param_1 + 0x164) = 0;
  }
  return;
}

