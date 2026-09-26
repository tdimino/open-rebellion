
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void * __thiscall FUN_00439320(void *this,undefined4 param_1,undefined4 param_2)

{
  int iVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d82e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0049dcf0((undefined4 *)((int)this + 0x18));
  local_4 = 0;
  FUN_0049c780((undefined4 *)((int)this + 0x3c));
  local_4._0_1_ = 1;
  FUN_0049c6a0((undefined4 *)((int)this + 0x138));
  local_4._0_1_ = 2;
  FUN_005f4950((void *)((int)this + 0x150),0);
  local_4._0_1_ = 3;
  FUN_005f5ca0((undefined4 *)((int)this + 0x154));
  local_4._0_1_ = 4;
  FUN_00520570((undefined4 *)((int)this + 0x174));
  *(undefined4 *)((int)this + 4) = 1;
  *(undefined4 *)((int)this + 0x10) = 1;
  *(undefined4 *)((int)this + 0xc) = 1;
  *(undefined4 *)((int)this + 0x144) = param_2;
  local_4 = CONCAT31(local_4._1_3_,5);
  *(undefined ***)this = &PTR_FUN_00658e10;
  *(undefined4 *)((int)this + 0x14) = param_1;
  *(undefined1 *)((int)this + 0x170) = 8;
  *(undefined4 *)((int)this + 0x38) = 0;
  *(undefined4 *)((int)this + 0x34) = 0;
  FUN_0049c880((undefined4 *)((int)this + 0x3c),param_1);
  *(undefined4 *)((int)this + 0x24) = param_1;
  *(undefined4 *)((int)this + 0x160) = 0;
  DAT_006b28b8 = 0;
  *(undefined4 *)((int)this + 0x164) = 0;
  *(undefined4 *)((int)this + 0x168) = 0;
  *(undefined4 *)((int)this + 0x16c) = 0;
  DAT_006b28d0 = 0;
  DAT_006b28cc = 0;
  *(undefined4 *)((int)this + 0x148) = 0;
  *(undefined4 *)((int)this + 0x14c) = 8;
  DAT_006b28bc = 0;
  _DAT_006b28c0 = 0;
  DAT_006b28b0 = 0;
  DAT_006b28c4 = 0;
  DAT_006b28c8 = 0;
  *(undefined4 *)((int)this + 0x17c) = 0;
  *(undefined4 *)((int)this + 0x174) = 0;
  *(undefined4 *)((int)this + 0x178) = 0;
  *(undefined4 *)((int)this + 0x184) = 0;
  *(undefined4 *)((int)this + 0x188) = 0;
  *(undefined4 *)((int)this + 0x18c) = 0;
  *(undefined4 *)((int)this + 400) = 0;
  iVar1 = FUN_00401b00();
  if ((iVar1 != 1) && (iVar1 != 4)) {
    DAT_006b28b0 = DAT_006b28b0 | 0x8000;
  }
  ExceptionList = local_c;
  return this;
}

