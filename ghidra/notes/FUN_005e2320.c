
void __fastcall FUN_005e2320(int param_1)

{
  uint *this;
  void *this_00;
  uint *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654edb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this = (uint *)FUN_005ff440(param_1);
  if (this != (uint *)0x0) {
    this_00 = (void *)FUN_00618b70(0x28);
    local_4 = 0;
    if (this_00 == (void *)0x0) {
      puVar1 = (uint *)0x0;
    }
    else {
      puVar1 = (uint *)FUN_005fbda0(this_00,0x66,0x6b,this);
    }
    local_4 = 0xffffffff;
    *(uint **)(param_1 + 0xa0) = puVar1;
    if (puVar1 != (uint *)0x0) {
      FUN_005fcc30(this,puVar1,0,0,0x17,0x3b,(undefined4 *)0x66,0x6b);
    }
  }
  ExceptionList = local_c;
  return;
}

