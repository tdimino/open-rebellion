
void __thiscall FUN_0041b110(void *this,uint *param_1)

{
  bool bVar1;
  void *pvVar2;
  uint local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b558;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(local_18);
  local_4 = 0;
  local_18[1] = 8;
  local_18[2] = 0x10;
  if ((*param_1 >> 0x18 < 8) || (0xf < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    FUN_004f26d0(&stack0xffffffd8,param_1);
    pvVar2 = FUN_00419e40((void *)((int)this + 0x78));
    if (pvVar2 != (void *)0x0) {
      FUN_004f26d0(local_18,(undefined4 *)((int)pvVar2 + 0x28));
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
        FUN_004f26d0(&stack0xffffffd8,local_18);
        pvVar2 = FUN_00403d30((void *)((int)this + 0x2c));
        if (pvVar2 != (void *)0x0) {
          FUN_00433e30((int)pvVar2);
        }
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

