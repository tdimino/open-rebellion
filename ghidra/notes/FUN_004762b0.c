
void * __thiscall FUN_004762b0(void *this,undefined4 param_1,undefined4 param_2)

{
  int iVar1;
  undefined4 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632ea9;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590(this);
  local_4 = 0;
  FUN_004f5510((undefined4 *)((int)this + 0x24));
  local_4._0_1_ = 1;
  FUN_004f5510((undefined4 *)((int)this + 0x2c));
  local_4._0_1_ = 2;
  FUN_004ece30((undefined4 *)((int)this + 0x34));
  local_4._0_1_ = 3;
  FUN_004ece30((undefined4 *)((int)this + 0x38));
  local_4._0_1_ = 4;
  FUN_004ece30((undefined4 *)((int)this + 0x3c));
  local_4._0_1_ = 5;
  FUN_004ece30((undefined4 *)((int)this + 0x40));
  local_4._0_1_ = 6;
  FUN_004355d0((undefined4 *)((int)this + 0x48));
  local_4._0_1_ = 7;
  FUN_005f4950((void *)((int)this + 0x60),0);
  local_4._0_1_ = 8;
  FUN_005f4950((void *)((int)this + 100),0);
  local_4._0_1_ = 9;
  FUN_005f52c0((undefined4 *)((int)this + 0xb0));
  local_4 = CONCAT31(local_4._1_3_,10);
  FUN_005f52c0((undefined4 *)((int)this + 0xd4));
  *(undefined4 *)((int)this + 0x20) = param_1;
  *(undefined4 *)((int)this + 0x58) = param_2;
  *(undefined ***)this = &PTR_FUN_0065a3b0;
  *(undefined4 *)((int)this + 0x1c) = 0;
  *(undefined4 *)((int)this + 0x5c) = 0;
  *(undefined4 *)((int)this + 0x44) = 0;
  *(undefined4 *)((int)this + 0x9c) = 0;
  puVar2 = (undefined4 *)((int)this + 0x6c);
  for (iVar1 = 10; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  *(undefined4 *)((int)this + 0x94) = 0;
  *(undefined4 *)((int)this + 0x98) = 0;
  *(undefined4 *)((int)this + 0xa0) = 0;
  *(undefined4 *)((int)this + 0xa4) = 0;
  *(undefined4 *)((int)this + 0xa8) = 0;
  *(undefined4 *)((int)this + 0xac) = 0;
  *(undefined4 *)((int)this + 0xbc) = 0;
  *(undefined4 *)((int)this + 0xc0) = 0;
  *(undefined4 *)((int)this + 0xc4) = 0;
  *(undefined4 *)((int)this + 200) = 0;
  *(undefined4 *)((int)this + 0xcc) = 0;
  *(undefined4 *)((int)this + 0xd0) = 0;
  *(undefined4 *)((int)this + 0xe0) = 0;
  *(undefined4 *)((int)this + 0xe4) = 0;
  *(undefined4 *)((int)this + 0x68) = 0;
  ExceptionList = local_c;
  return this;
}

