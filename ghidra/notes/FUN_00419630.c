
void __thiscall FUN_00419630(void *this,uint *param_1)

{
  bool bVar1;
  void *pvVar2;
  uint local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b1b8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(local_18);
  local_4 = 0;
  FUN_00403a30((void *)((int)this + 0x8c),param_1);
  pvVar2 = FUN_004f5940((void *)((int)this + 0x8c),param_1);
  if (pvVar2 != (void *)0x0) {
    FUN_004f26d0(local_18,(undefined4 *)((int)pvVar2 + 0x38));
    local_18[1] = 0x90;
    local_18[2] = 0x98;
    if ((local_18[0] >> 0x18 < 0x90) || (0x97 < local_18[0] >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      FUN_00431170((void *)((int)this + 0x2c),local_18);
    }
    else {
      local_18[1] = 0x14;
      local_18[2] = 0x1c;
      if ((local_18[0] >> 0x18 < 0x14) || (0x1b < local_18[0] >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        FUN_004306e0((void *)((int)this + 0x58),local_18);
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

